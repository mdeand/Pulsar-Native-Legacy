use imgui::*;
use crate::game_engine_ui::GameEngineUI;

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
        self.engine_ui.render(ui);
    }
}