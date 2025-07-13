use gpui::{ElementId, GlobalElementId, WindowContext, LayoutId, Bounds, Pixels, Corners, ImageData};
use crate::components::tab_system::{TabContentProvider, RegisterableTab};
use std::sync::Arc;
use gpui::{
    div, rgb, AnyElement, IntoElement, ParentElement, Styled, InteractiveElement, px
};
use gpui::prelude::*;

// --- Content Provider ---
// This struct defines the actual content and appearance of the level editor tab.
#[derive(Clone)]
pub struct LevelEditorContentProvider {
    title: String,
    level_data: String,
    is_dirty: bool,
}

impl LevelEditorContentProvider {
    pub fn new(title: String) -> Self {
        Self {
            title,
            level_data: "Level data goes here...".to_string(),
            is_dirty: false,
        }
    }
}

impl TabContentProvider for LevelEditorContentProvider {
    fn render_content(&self, _tab_id: usize) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xE0E0E0))
                            .child(self.title.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x2A2A2A))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(rgb(0x333333)))
                                    .text_color(rgb(0xE0E0E0))
                                    .child("Save")
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x2A2A2A))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(rgb(0x333333)))
                                    .text_color(rgb(0xE0E0E0))
                                    .child("Load")
                            )
                    )
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .gap_4()
                    .child(
                        // Tools Panel
                        div()
                            .w(px(208.0))
                            .bg(rgb(0x2A2A2A))
                            .rounded_md()
                            .p_3()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xE0E0E0))
                                    .mb_3()
                                    .child("Tools")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(div().px_3().py_2().bg(rgb(0x404040)).rounded_md().cursor_pointer().hover(|s| s.bg(rgb(0x4A4A4A))).text_color(rgb(0xE0E0E0)).child("🖌️ Brush"))
                                    .child(div().px_3().py_2().bg(rgb(0x333333)).rounded_md().cursor_pointer().hover(|s| s.bg(rgb(0x404040))).text_color(rgb(0xB0B0B0)).child("✏️ Pencil"))
                                    .child(div().px_3().py_2().bg(rgb(0x333333)).rounded_md().cursor_pointer().hover(|s| s.bg(rgb(0x404040))).text_color(rgb(0xB0B0B0)).child("🧹 Eraser"))
                            )
                    )
                    .child(
                        // Animated Viewport
                        div()
                            .flex_1()
                            .child(MySurface::new().into_element())
                    )
                    .child(
                        // Properties Panel
                        div()
                            .w(px(208.0))
                            .bg(rgb(0x2A2A2A))
                            .rounded_md()
                            .p_3()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xE0E0E0))
                                    .mb_3()
                                    .child("Properties")
                            )
                            // Add properties here
                    )
            )
            .into_any_element()
    }
    
    fn get_title(&self) -> String {
        self.title.clone()
    }
    
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    
    fn can_close(&self) -> bool {
        // For a real app, you might check if there are unsaved changes.
        true
    }
}


// --- Animated Framebuffer Viewport ---
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Instant;
use image::RgbaImage;
use rayon::prelude::*;

static ANIMATION_START_TIME: Lazy<Instant> = Lazy::new(Instant::now);
static PAINT_TIMES: Lazy<Mutex<Vec<u128>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(60)));
static PAINT_LAST_60: Lazy<Mutex<Option<u128>>> = Lazy::new(|| Mutex::new(None));

pub struct MySurface {
    framebuffer: Arc<Mutex<RgbaImage>>,
    width: u32,
    height: u32,
}

impl MySurface {
    pub fn new() -> Self {
        let width = 400;
        let height = 300;
        Self {
            framebuffer: Arc::new(Mutex::new(RgbaImage::new(width, height))),
            width,
            height,
        }
    }
}

impl Element for MySurface {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // Use measured layout, but let the parent div control the size
        (
            cx.request_measured_layout(Default::default(), |_, requested_size, _| {
                use gpui::AvailableSpace;
                gpui::Size {
                    width: Pixels(match requested_size.width {
                        gpui::AvailableSpace::Definite(px) => px.0,
                        _ => 400.0,
                    }),
                    height: Pixels(match requested_size.height {
                        gpui::AvailableSpace::Definite(px) => px.0,
                        _ => 300.0,
                    }),
                }
            }),
            (),
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        let start = Instant::now();
        let elapsed_time = ANIMATION_START_TIME.elapsed().as_secs_f32();
        let width = bounds.size.width.to_f64() as u32;
        let height = bounds.size.height.to_f64() as u32;
        // Always resize framebuffer to match parent div's size
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            let mut fb = self.framebuffer.lock().unwrap();
            *fb = RgbaImage::new(width.max(1), height.max(1));
        }
        let speed = 50.0;
        let strip_width = 30.0;
        let time_offset = elapsed_time * speed;
        {
            let mut fb = self.framebuffer.lock().unwrap();
            let buf: &mut [u8] = fb.as_mut();
            buf.par_chunks_mut(4).enumerate().for_each(|(i, px)| {
                let x = (i % width.max(1) as usize) as f32;
                let y = (i / width.max(1) as usize) as f32;
                let position = (x + time_offset) % (strip_width * 3.0);
                let (r, g, b) = if position < strip_width {
                    let t = position / strip_width;
                    let r = ((1.0 - t) * 255.0) as u8;
                    let g = (t * 255.0) as u8;
                    (r, g, 0)
                } else if position < strip_width * 2.0 {
                    let t = (position - strip_width) / strip_width;
                    let g = ((1.0 - t) * 255.0) as u8;
                    let b = (t * 255.0) as u8;
                    (0, g, b)
                } else {
                    let t = (position - strip_width * 2.0) / strip_width;
                    let b = ((1.0 - t) * 255.0) as u8;
                    let r = (t * 255.0) as u8;
                    (r, 0, b)
                };
                let brightness = if height > 1 {
                    0.7 + 0.3 * (y / (height as f32 - 1.0))
                } else {
                    1.0
                };
                px[0] = ((r as f32) * brightness) as u8;
                px[1] = ((g as f32) * brightness) as u8;
                px[2] = ((b as f32) * brightness) as u8;
                px[3] = 255;
            });
        }
        let fb = self.framebuffer.lock().unwrap();
        let _ = cx.paint_image(
            bounds,
            Corners::all(Pixels(10.)),
            Arc::new(ImageData::new(fb.clone())),
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
        cx.refresh();
    }
}

impl IntoElement for MySurface {
    type Element = MySurface;
    fn into_element(self) -> Self::Element {
        self
    }
}

// --- Registrable Type ---
// This is the "factory" for the Level Editor tab.
// You will register this struct in your main application setup.
pub struct LevelEditorTabType;

impl RegisterableTab for LevelEditorTabType {
    fn name(&self) -> &'static str {
        "Level Editor"
    }

    fn create(&self) -> Arc<dyn TabContentProvider> {
        Arc::new(LevelEditorContentProvider::new("New Level".to_string()))
    }

    fn icon(&self) -> Option<String> {
        Some("🎮".to_string())
    }
}
