use std::{thread::sleep, time::Duration};
#[cfg(windows)]
use windows_sys::Win32::System::Threading::{SetPriorityClass, GetCurrentProcess, HIGH_PRIORITY_CLASS};

use gpui::{*, App as Application};

mod app;
mod components;
mod frame_counter;

#[tokio::main]
async fn main() {
    // Set high process priority on Windows
    #[cfg(windows)]
    unsafe {
        let handle = GetCurrentProcess();
        SetPriorityClass(handle, HIGH_PRIORITY_CLASS);
    }
    
    let app = Application::new();
    
    app.background_executor().spawn((async || loop {
        sleep(Duration::from_secs(1));
        println!("testing");
    })()).detach();

    app.run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    title: Some("Pulsar Engine".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| {
                app::App::new(cx)
            },
        )
        .unwrap();

        // Adaptive frame rate system with smooth pacing
        cx.spawn(|mut cx| async move {
            println!("=== ADAPTIVE SMOOTH FRAME PACING ===");
            
            // Start with a reasonable target and adapt
            let mut target_fps = 240u64;
            let mut target_frame_time = Duration::from_nanos(1_000_000_000 / target_fps);
            
            let mut last_stats = std::time::Instant::now();
            let mut frames = 0u64;
            let mut computation_frames = 0u64;
            let mut next_frame_time = std::time::Instant::now();
            
            // Frame time tracking for consistency analysis
            let mut frame_times = std::collections::VecDeque::with_capacity(120);
            let mut missed_frames = 0u64;
            let mut adaptation_timer = std::time::Instant::now();
            
            // Adaptive timing strategy
            let mut use_spin_wait = true;
            let mut frame_skip_threshold = Duration::from_millis(50); // 20 FPS minimum
            
            loop {
                let frame_start = std::time::Instant::now();
                
                // Check if we're falling behind and should skip this frame
                let behind_schedule = frame_start > next_frame_time + frame_skip_threshold;
                
                if behind_schedule {
                    missed_frames += 1;
                    // Reset timing to current time to avoid spiraling
                    next_frame_time = frame_start;
                    continue;
                }

                // Computation work
                let comp_start = std::time::Instant::now();
                {
                    let mut frame = crate::frame_counter::GLOBAL_FRAME_COUNTER.lock().unwrap();
                    *frame += 1;
                }
                computation_frames += 1;
                let comp_time = comp_start.elapsed();

                // Adaptive UI refresh rate based on performance
                let ui_start = std::time::Instant::now();
                let ui_refresh_interval = if target_fps > 120 { 2 } else { 1 };
                let ui_time = if frames % ui_refresh_interval == 0 {
                    cx.update(|cx| {
                        cx.refresh();
                    }).ok();
                    ui_start.elapsed()
                } else {
                    Duration::ZERO
                };

                frames += 1;
                let frame_elapsed = frame_start.elapsed();
                
                // Track frame time consistency
                frame_times.push_back(frame_elapsed.as_nanos());
                if frame_times.len() > 120 {
                    frame_times.pop_front();
                }

                // Adaptive performance monitoring every 2 seconds
                if adaptation_timer.elapsed().as_secs_f32() >= 2.0 {
                    let frame_times_vec: Vec<u128> = frame_times.iter().cloned().collect();
                    if frame_times_vec.len() > 10 {
                        let avg_frame_time = frame_times_vec.iter().sum::<u128>() / frame_times_vec.len() as u128;
                        let variance = frame_times_vec.iter()
                            .map(|&x| {
                                let diff = x as i128 - avg_frame_time as i128;
                                (diff * diff) as u128
                            })
                            .sum::<u128>() / frame_times_vec.len() as u128;
                        let std_dev = (variance as f64).sqrt();
                        
                        // Calculate frame consistency score (lower is better)
                        let consistency_score = std_dev / avg_frame_time as f64;
                        let avg_frame_time_ms = avg_frame_time as f64 / 1_000_000.0;
                        let std_dev_ms = std_dev / 1_000_000.0;
                        
                        println!(
                            "ADAPTATION: Avg: {:.2}ms | StdDev: {:.2}ms | Consistency: {:.3} | Missed: {} | FPS: {}",
                            avg_frame_time_ms,
                            std_dev_ms,
                            consistency_score,
                            missed_frames,
                            target_fps
                        );
                        
                        // Adaptive thresholds based on current FPS
                        let max_acceptable_std_dev = if target_fps >= 200 { 2.0 } else { 3.0 }; // Stricter at higher FPS
                        let max_consistency_score = if target_fps >= 200 { 1.0 } else { 1.5 };
                        
                        // Reduce FPS only if really struggling
                        if (consistency_score > max_consistency_score || std_dev_ms > max_acceptable_std_dev || missed_frames > 30) && target_fps > 60 {
                            let reduction = if missed_frames > 50 { 40 } else { 20 }; // Bigger drop if really bad
                            target_fps = (target_fps.saturating_sub(reduction)).max(60);
                            target_frame_time = Duration::from_nanos(1_000_000_000 / target_fps);
                            println!("ADAPTED: Reducing target FPS to {} (consistency: {:.2}, std_dev: {:.2}ms, missed: {})", 
                                target_fps, consistency_score, std_dev_ms, missed_frames);
                        } 
                        // Be more aggressive about increasing FPS
                        else if target_fps < 240 {
                            let can_increase = if target_fps < 120 {
                                // Below 120 FPS - be very liberal about increasing
                                consistency_score < 1.0 && std_dev_ms < 4.0 && missed_frames < 10
                            } else if target_fps < 180 {
                                // 120-180 FPS - moderately strict
                                consistency_score < 0.8 && std_dev_ms < 2.5 && missed_frames < 5
                            } else {
                                // Above 180 FPS - be strict about quality
                                consistency_score < 0.5 && std_dev_ms < 1.5 && missed_frames == 0
                            };
                            
                            if can_increase {
                                let increase = if target_fps < 120 { 30 } else if target_fps < 180 { 20 } else { 10 };
                                target_fps = (target_fps + increase).min(240);
                                target_frame_time = Duration::from_nanos(1_000_000_000 / target_fps);
                                println!("ADAPTED: Increasing target FPS to {} (performance is good)", target_fps);
                            } else {
                                println!("STABLE: Maintaining {} FPS (consistency: {:.2}, std_dev: {:.2}ms, missed: {})", 
                                    target_fps, consistency_score, std_dev_ms, missed_frames);
                            }
                        }
                        // At max FPS
                        else {
                            println!("MAX: At {} FPS limit (consistency: {:.2}, std_dev: {:.2}ms, missed: {})", 
                                target_fps, consistency_score, std_dev_ms, missed_frames);
                        }
                        
                        missed_frames = 0;
                    }
                    adaptation_timer = std::time::Instant::now();
                }

                // Print stats every second
                if last_stats.elapsed().as_secs_f32() >= 1.0 {
                    let actual_fps = frames as f64 / last_stats.elapsed().as_secs_f64();
                    let computation_fps = computation_frames as f64 / last_stats.elapsed().as_secs_f64();
                    
                    // Calculate frame time statistics
                    let frame_times_vec: Vec<u128> = frame_times.iter().cloned().collect();
                    let (min_time, max_time, avg_time) = if !frame_times_vec.is_empty() {
                        let min = *frame_times_vec.iter().min().unwrap() as f64 / 1_000_000.0;
                        let max = *frame_times_vec.iter().max().unwrap() as f64 / 1_000_000.0;
                        let avg = frame_times_vec.iter().sum::<u128>() as f64 / frame_times_vec.len() as f64 / 1_000_000.0;
                        (min, max, avg)
                    } else {
                        (0.0, 0.0, 0.0)
                    };
                    
                    println!(
                        "Target: {} FPS | Actual: {:.1} FPS | Comp: {:.1} FPS | Frame: {:.3}ms [{:.2}-{:.2}ms] | Comp: {}μs | UI: {}μs",
                        target_fps,
                        actual_fps,
                        computation_fps,
                        avg_time,
                        min_time,
                        max_time,
                        comp_time.as_nanos() / 1000,
                        ui_time.as_nanos() / 1000
                    );
                    frames = 0;
                    computation_frames = 0;
                    last_stats = std::time::Instant::now();
                }

                // Smart timing strategy for smooth pacing
                next_frame_time += target_frame_time;
                let now = std::time::Instant::now();
                
                if next_frame_time > now {
                    let sleep_time = next_frame_time - now;
                    
                    if sleep_time < Duration::from_micros(100) {
                        // Very short delay - spin wait for precision
                        while std::time::Instant::now() < next_frame_time {
                            std::hint::spin_loop();
                        }
                    } else if sleep_time < Duration::from_micros(1000) && use_spin_wait {
                        // Short delay - hybrid approach
                        let spin_start = next_frame_time - Duration::from_micros(200);
                        if sleep_time > Duration::from_micros(300) {
                            std::thread::sleep(sleep_time - Duration::from_micros(200));
                        }
                        while std::time::Instant::now() < next_frame_time {
                            std::hint::spin_loop();
                        }
                    } else {
                        // Longer delay - use async timer
                        cx.background_executor().timer(sleep_time).await;
                    }
                } else {
                    // We're behind - don't sleep, just continue
                    next_frame_time = now;
                }
            }
        }).detach();
    })
}