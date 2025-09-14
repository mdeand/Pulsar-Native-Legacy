use imgui::*;
use crate::tab_system::TabContent;
use std::time::Instant;
use rayon::prelude::*;
use image::{RgbaImage, Rgba};

pub struct LevelEditor {
    title: String,
    is_dirty: bool,

    // Viewport state
    viewport_texture: Option<TextureId>,
    viewport_size: [u32; 2],
    animation_start: Instant,

    // Level data
    level_data: String,

    // Tools
    selected_tool: Tool,
    brush_size: f32,
}

#[derive(PartialEq, Clone)]
enum Tool {
    Brush,
    Pencil,
    Eraser,
}

impl LevelEditor {
    pub fn new(title: String) -> Self {
        Self {
            title,
            is_dirty: false,
            viewport_texture: None,
            viewport_size: [800, 600],
            animation_start: Instant::now(),
            level_data: "Level data goes here...".to_string(),
            selected_tool: Tool::Brush,
            brush_size: 10.0,
        }
    }

    fn render_animated_viewport(&mut self, ui: &Ui, size: [f32; 2]) -> bool {
        // Create or update the texture for our animated content
        let width = size[0] as u32;
        let height = size[1] as u32;

        if width == 0 || height == 0 {
            return false;
        }

        // Generate animated content
        let elapsed = self.animation_start.elapsed().as_secs_f32();
        let mut image = RgbaImage::new(width, height);
        let speed = 50.0;
        let strip_width = 30.0;
        let time_offset = elapsed * speed;

        // Use rayon for parallel pixel computation
        let pixels: Vec<Rgba<u8>> = (0..width * height)
            .into_par_iter()
            .map(|i| {
                let x = (i % width) as f32;
                let y = (i / width) as f32;
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

                Rgba([
                    ((r as f32) * brightness) as u8,
                    ((g as f32) * brightness) as u8,
                    ((b as f32) * brightness) as u8,
                    255,
                ])
            })
            .collect();

        // Set pixels in the image
        for (i, pixel) in pixels.iter().enumerate() {
            let x = i as u32 % width;
            let y = i as u32 / width;
            image.put_pixel(x, y, *pixel);
        }

        // For now, we'll just draw a colored rectangle since imgui-rs texture management
        // is more complex and would require additional setup
        let draw_list = ui.get_window_draw_list();
        let canvas_pos = ui.cursor_screen_pos();
        let canvas_size = size;

        // Draw animated background
        let hue = (elapsed * 0.5) % 1.0;
        let color = [
            hue,
            1.0 - hue,
            (elapsed * 2.0) % 1.0,
            0.5
        ];

        draw_list
            .add_rect(
                canvas_pos,
                [canvas_pos[0] + canvas_size[0], canvas_pos[1] + canvas_size[1]],
                color,
            )
            .filled(true)
            .build();

        // Add some animated elements
        let center_x = canvas_pos[0] + canvas_size[0] * 0.5;
        let center_y = canvas_pos[1] + canvas_size[1] * 0.5;
        let radius = 50.0 + 20.0 * (elapsed * 3.0).sin();

        draw_list
            .add_circle(
                [center_x, center_y],
                radius,
                (255.0, 255.0, 255.0, 200.0),
            )
            .num_segments(32)
            .build();

        ui.dummy(canvas_size);

        true
    }
}

impl TabContent for LevelEditor {
    fn render(&mut self, ui: &Ui) {
        // Header with title and actions
        ui.columns(2, "LevelEditorHeader", false);
        ui.set_column_width(0, -150.0); // Reserve 150px for buttons on the right

        ui.text(&format!("Level Editor: {}", self.title));

        ui.next_column();

        // Action buttons
        if ui.button("Save") {
            // Handle save
            self.is_dirty = false;
        }
        ui.same_line();
        if ui.button("Load") {
            // Handle load
        }

        ui.columns(1, "", false);
        ui.separator();

        // Main layout: tools | viewport | properties
        ui.columns(3, "LevelEditorMain", false);
        ui.set_column_width(0, 200.0);  // Tools panel
        ui.set_column_width(1, -200.0); // Viewport (remaining space minus properties)

        // Left Panel: Tools
        ui.text("Tools");
        ui.separator();

        let tools = [(Tool::Brush, "🖌️ Brush"), (Tool::Pencil, "✏️ Pencil"), (Tool::Eraser, "🧹 Eraser")];

        for (tool, label) in &tools {
            let selected = self.selected_tool == *tool;
            if ui.radio_button_bool(label, selected) {
                self.selected_tool = tool.clone();
            }
        }

        ui.separator();
        ui.text("Brush Settings");
        ui.slider_config("Size", 1.0, 100.0)
            .build(&mut self.brush_size);

        ui.next_column();

        // Middle: Animated Viewport
        ui.text("Viewport");
        ui.separator();

        let viewport_size = ui.content_region_avail();
        let viewport_size = [viewport_size[0], viewport_size[1] - 20.0]; // Reserve some space

        if viewport_size[0] > 0.0 && viewport_size[1] > 0.0 {
            self.render_animated_viewport(ui, viewport_size);
        }

        ui.next_column();

        // Right Panel: Properties
        ui.text("Properties");
        ui.separator();

        ui.text("Level Information");
        ui.text(&format!("Size: {}x{}", self.viewport_size[0], self.viewport_size[1]));

        ui.separator();
        ui.text("Selected Object");
        ui.text("No object selected");

        // Level data editor
        ui.separator();
        ui.text("Level Data");
        if ui.input_text_multiline("##leveldata", &mut self.level_data, [0.0, 100.0])
            .build()
        {
            self.is_dirty = true;
        }

        ui.columns(1, "", false);
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn get_icon(&self) -> Option<&str> {
        Some("🎮")
    }
}