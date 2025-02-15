use gpui::{div, rgb, IntoElement, ParentElement, Styled, ViewContext};
use super::super::editor_plugin::{EditorMetadata, EditorView};
use crate::app::App;
use crate::components::tabs_bar::TabBar;

#[derive(Clone)]
pub struct LevelEditor;
pub struct LevelEditorView;

impl EditorMetadata for LevelEditor {
    fn name(&self) -> &'static str { "Level" }
    fn icon(&self) -> &'static str { "🗺️" }
    fn title(&self) -> &'static str { "Level Editor" }
    fn description(&self) -> &'static str { "Edit levels and game worlds." }
    
    fn create_view(&self, _cx: &mut ViewContext<TabBar>) -> impl EditorView {
        LevelEditorView
    }
}

impl EditorView for LevelEditorView {
    fn render(&self, _cx: &mut ViewContext<TabBar>) -> impl gpui::Element {
        div()
            .text_color(rgb(0xE0E0E0))
            .child("Level Editor Content")
            .size_full()
    }
}