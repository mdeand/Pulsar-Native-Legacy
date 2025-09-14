use imgui::*;
use crate::{tab_system::TabSystem, frame_counter};

pub struct App {
    tab_system: TabSystem,
}

impl App {
    pub fn new() -> Self {
        let mut tab_system = TabSystem::new();

        // Register default tabs
        tab_system.register_tab_type("Level Editor", "🎮", Box::new(|| {
            Box::new(crate::level_editor::LevelEditor::new("New Level".to_string())) as Box<dyn crate::tab_system::TabContent>
        }));

        tab_system.register_tab_type("Text Editor", "📝", Box::new(|| {
            Box::new(crate::tab_system::TextEditor::new("New Document".to_string())) as Box<dyn crate::tab_system::TabContent>
        }));

        tab_system.register_tab_type("Settings", "⚙️", Box::new(|| {
            Box::new(crate::tab_system::Settings::new()) as Box<dyn crate::tab_system::TabContent>
        }));

        // Add a default tab
        tab_system.add_tab("Level Editor", crate::level_editor::LevelEditor::new("Default Level".to_string()));

        Self { tab_system }
    }

    pub fn run(&mut self, ui: &Ui) {
        // Simple test - just display basic info
        ui.window("Test Window")
            .size([300.0, 200.0], Condition::Always)
            .position([50.0, 50.0], Condition::Always)
            .build(|| {
                ui.text("Hello, Pulsar Engine!");
                ui.text("This is a simple test window");

                let fps = frame_counter::get_fps();
                ui.text(&format!("FPS: {}", fps));

                if ui.button("Test Button") {
                    // Test button click
                }
            });
    }
}