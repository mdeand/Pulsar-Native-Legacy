use gpui::{div, rgb, ParentElement, Styled, ViewContext};
use super::super::editor_plugin::{EditorMetadata, EditorView};
use crate::components::tabs_bar::TabBar;

#[derive(Clone)]
pub struct LevelEditor;
pub struct LevelEditorView {
    random_number: u32,
}

impl EditorMetadata for LevelEditor {
    fn name(&self)        -> &'static str { "Level" }
    fn icon(&self)        -> &'static str { "🗺️" }
    fn title(&self)       -> &'static str { "Level Editor" }
    fn description(&self) -> &'static str { "Edit levels and game worlds." }
    
    fn create_view(&self, _cx: &mut ViewContext<TabBar>) -> impl EditorView {
        LevelEditorView {
            random_number: rand::random::<u32>() % 1000,
        }
    }
}

impl EditorView for LevelEditorView {
    fn render(&self, _cx: &mut ViewContext<TabBar>) -> impl gpui::Element {
        div()
            .text_color(rgb(0x555555))
            .child(format!("Level Editor View: {}", self.random_number))
            .size_full()
    }
}