use imgui::*;
use crate::ui::SimpleGameUI;

pub struct GameEngineUI {
    simple_ui: SimpleGameUI,
}

impl GameEngineUI {
    pub fn new() -> Self {
        Self {
            simple_ui: SimpleGameUI::new(),
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        self.simple_ui.render(ui);
    }
}