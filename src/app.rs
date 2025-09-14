use imgui::*;
use crate::{game_engine_ui::GameEngineUI, frame_counter};

pub struct App {
    engine_ui: GameEngineUI,
}

impl App {
    pub fn new() -> Self {
        Self {
            engine_ui: GameEngineUI::new()
        }
    }

    pub fn run(&mut self, ui: &Ui) {
        let fps = frame_counter::get_fps();
        self.engine_ui.render(ui, fps);
    }
}