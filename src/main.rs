use std::time::Instant;
use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(windows)]
use windows_sys::Win32::System::Threading::{SetPriorityClass, GetCurrentProcess, HIGH_PRIORITY_CLASS};

mod app;
mod frame_counter;
mod tab_system;
mod level_editor;
mod game_engine_ui;

use app::App;

#[tokio::main]
async fn main() {
    // Set high process priority on Windows
    #[cfg(windows)]
    unsafe {
        let handle = GetCurrentProcess();
        SetPriorityClass(handle, HIGH_PRIORITY_CLASS);
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pulsar Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 800))
        .build(&event_loop)
        .unwrap();

    // Set up wgpu
    let size = window.inner_size();
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&window).unwrap() };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    config.format = wgpu::TextureFormat::Bgra8UnormSrgb;
    config.usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
    surface.configure(&device, &config);

    // Set up imgui
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    // Configure AMOLED black theme
    {
        let style = imgui.style_mut();

        // Basic AMOLED black colors for imgui 0.10.0
        style.window_rounding = 0.0;
        style.frame_rounding = 2.0;
        style.grab_rounding = 2.0;
        style.scrollbar_rounding = 2.0;
        style.window_padding = [4.0, 4.0];
        style.frame_padding = [8.0, 4.0];
        style.item_spacing = [4.0, 4.0];
        style.item_inner_spacing = [4.0, 4.0];
        style.indent_spacing = 16.0;
        style.scrollbar_size = 12.0;
        style.grab_min_size = 8.0;
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);

    let hidpi_factor = window.scale_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        },
    ]);

    let renderer_config = RendererConfig {
        texture_format: config.format,
        ..Default::default()
    };
    let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    let mut app = App::new();

    // Frame timing setup
    let mut last_frame = Instant::now();
    let mut frame_count = 0u64;
    let mut fps_counter = Instant::now();

    event_loop.run(move |event, _target, control_flow| {
            *control_flow = ControlFlow::Poll;
            platform.handle_event(imgui.io_mut(), &window, &event);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    config.width = size.width.max(1);
                    config.height = size.height.max(1);
                    surface.configure(&device, &config);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;

                    let frame = match surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("dropped frame: {e:?}");
                            return;
                        }
                    };

                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");

                    let ui = imgui.frame();

                    // Update frame counter
                    frame_count += 1;
                    if fps_counter.elapsed().as_secs() >= 1 {
                        frame_counter::update_fps(frame_count);
                        frame_count = 0;
                        fps_counter = Instant::now();
                    }

                    // Run the app - check for valid display size to avoid ClipRect assertion
                    let io = ui.io();
                    if io.display_size[0] > 10.0 && io.display_size[1] > 10.0 {
                        app.run(&ui);
                    }

                    let mut encoder: wgpu::CommandEncoder = device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor { label: None }
                    );

                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.06,
                                    g: 0.06,
                                    b: 0.06,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    let draw_data = imgui.render();
                    renderer
                        .render(&draw_data, &queue, &device, &mut rpass)
                        .expect("Rendering failed");

                    drop(rpass);

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
}