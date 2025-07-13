use rayon::prelude::*;
use std::sync::Arc;

use gpui::ViewContext;
use image::RgbaImage;

use crate::components::editor_plugin::{EditorMetadata, EditorView};
use crate::components::tabs_bar::TabBar;
use crate::frame_counter::{GLOBAL_AVERAGE_FRAME_TIME, GLOBAL_FRAME_COUNTER, GLOBAL_FRAME_TIME};
use gpui::*;

#[derive(Clone)]
struct MySurface {
    // No longer holds its own frame counter
}

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::{Duration, Instant};

static PAINT_TIMES: Lazy<Mutex<Vec<u128>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(60)));
static PAINT_LAST_60: Lazy<Mutex<Option<u128>>> = Lazy::new(|| Mutex::new(None));
static ANIMATION_START_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());

impl Element for MySurface {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (
            cx.request_measured_layout(Default::default(), |_, _, _| gpui::Size {
                width: gpui::Pixels::from(100.),
                height: gpui::Pixels::from(100.),
            }),
            (),
        )
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        let start = Instant::now();

        // Calculate delta time and total elapsed time
        let elapsed_time = ANIMATION_START_TIME.elapsed().as_secs_f32();

        // Animate a smooth RGB strip gradient
        let width = 100;
        let height = bounds.size.height.to_f64() as u32;
        let mut img = RgbaImage::new(width, height);
        let buf: &mut [u8] = img.as_mut();

        // Animation parameters
        let speed = 50.0; // pixels per second
        let strip_width = 30.0; // width of each color band
        let time_offset = elapsed_time * speed;

        buf.par_chunks_mut(4).enumerate().for_each(|(i, px)| {
            let x = (i % width as usize) as f32;
            let y = (i / width as usize) as f32;

            // Create smooth moving RGB strip
            let position = (x + time_offset) % (strip_width * 3.0);

            let (r, g, b) = if position < strip_width {
                // Red to Green transition
                let t = position / strip_width;
                let r = ((1.0 - t) * 255.0) as u8;
                let g = (t * 255.0) as u8;
                (r, g, 0)
            } else if position < strip_width * 2.0 {
                // Green to Blue transition
                let t = (position - strip_width) / strip_width;
                let g = ((1.0 - t) * 255.0) as u8;
                let b = (t * 255.0) as u8;
                (0, g, b)
            } else {
                // Blue to Red transition
                let t = (position - strip_width * 2.0) / strip_width;
                let b = ((1.0 - t) * 255.0) as u8;
                let r = (t * 255.0) as u8;
                (r, 0, b)
            };

            // Add some vertical gradient for visual interest
            let brightness = 0.7 + 0.3 * (y / (height as f32 - 1.0));

            px[0] = ((r as f32) * brightness) as u8;
            px[1] = ((g as f32) * brightness) as u8;
            px[2] = ((b as f32) * brightness) as u8;
            px[3] = 255;
        });

        let _ = cx.paint_image(
            bounds,
            Corners::all(Pixels(10.)),
            Arc::new(ImageData::new(img)),
            false,
        );

        let elapsed = start.elapsed().as_micros();
        let mut times = PAINT_TIMES.lock().unwrap();
        times.push(elapsed);
        if times.len() >= 60 {
            let sum: u128 = times.iter().sum();
            let avg = sum / times.len() as u128;
            *PAINT_LAST_60.lock().unwrap() = Some(avg);
            times.clear();
        }
    }
}

impl IntoElement for MySurface {
    type Element = MySurface;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Clone)]
pub struct LevelEditor;
pub struct LevelEditorView {
    pub wgpu_surface: MySurface,
}

impl LevelEditorView {
    pub fn new() -> Self {
        Self {
            wgpu_surface: MySurface {},
        }
    }
}

impl EditorMetadata for LevelEditor {
    fn name(&self) -> &'static str {
        "Level Editor"
    }
    fn icon(&self) -> &'static str {
        "Q"
    }
    fn title(&self) -> &'static str {
        "Level Editor"
    }
    fn description(&self) -> &'static str {
        "Level Editor"
    }

    fn create_view(&self, _cx: &mut ViewContext<TabBar>) -> Box<dyn EditorView + 'static> {
        let surface = MySurface {};
        Box::new(LevelEditorView {
            wgpu_surface: surface,
        })
    }

    fn clone_box(&self) -> Box<dyn EditorMetadata> {
        Box::new(self.clone())
    }
}

impl EditorView for LevelEditorView {
    fn render(&self, _cx: &mut ViewContext<TabBar>) -> AnyElement {
        let frame = {
            let lock = GLOBAL_FRAME_COUNTER.lock().unwrap();
            *lock
        };
        let frame_time = {
            let lock = GLOBAL_FRAME_TIME.lock().unwrap();
            *lock
        };
        let avg_frame_time = {
            let lock = GLOBAL_AVERAGE_FRAME_TIME.lock().unwrap();
            *lock
        };

        // Calculate delta time info
        let elapsed_time = ANIMATION_START_TIME.elapsed().as_secs_f32();
        let fps = if frame_time > 0 {
            1000.0 / frame_time as f32
        } else {
            0.0
        };

        // Paint timing stats every 60 frames
        let stats_60 = {
            let lock = PAINT_LAST_60.lock().unwrap();
            *lock
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .min_h_0()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .children([
                        div()
                            .child(format!("Frame: {}", frame))
                            .text_color(rgb(0xFFFFFF))
                            .text_lg()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x333333))
                            .rounded(Pixels(8.0))
                            .flex(),
                        div()
                            .child(format!("Last frame time: {} ms", frame_time))
                            .text_color(rgb(0xFFFFFF))
                            .text_lg()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x333333))
                            .rounded(Pixels(8.0))
                            .flex(),
                        div()
                            .child(format!("Avg frame time: {} ms", avg_frame_time))
                            .text_color(rgb(0xFFFFFF))
                            .text_lg()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x333333))
                            .rounded(Pixels(8.0))
                            .flex(),
                        div()
                            .child(format!("Current FPS: {:.1}", fps))
                            .text_color(rgb(0x00FF00))
                            .text_lg()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x003300))
                            .rounded(Pixels(8.0))
                            .flex(),
                    ])
                    .child(
                        div()
                            .child(format!("Elapsed time: {:.2}s", elapsed_time))
                            .text_color(rgb(0x00FFFF))
                            .text_lg()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x003333))
                            .rounded(Pixels(8.0)),
                    )
                    .child(
                        div()
                            .child(match stats_60 {
                                Some(us) => format!("60-frame paint time: {}μs", us),
                                None => "60-frame paint time: --".to_string(),
                            })
                            .text_color(rgb(0xFFFF00))
                            .text_lg()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x444400))
                            .rounded(Pixels(8.0)),
                    )
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .size_full()
                    .child(self.wgpu_surface.clone())
            )
            .h_full()
            .into_any()
        }
}
