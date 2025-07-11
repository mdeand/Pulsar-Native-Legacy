use gpui::{div, rgb, AnyElement, Element, ParentElement, Styled, ViewContext};
use super::super::editor_plugin::{EditorMetadata, EditorView};
use crate::components::tabs_bar::TabBar;

#[derive(Clone)]
pub struct ExampleEditor;
pub struct ExampleEditorView {
    random_number: u32,
}

impl EditorMetadata for ExampleEditor {
    fn name(&self)        -> &'static str { "Example" }
    fn icon(&self)        -> &'static str { "🗺️" }
    fn title(&self)       -> &'static str { "Example Editor" }
    fn description(&self) -> &'static str { "Editor for showing how editor tabs are created." }
    
    fn create_view(&self, _cx: &mut ViewContext<TabBar>) -> Box<(dyn EditorView + 'static)> {
        Box::new(ExampleEditorView {
            random_number: rand::random::<u32>() % 1000,
        })
    }
    
    fn clone_box(&self) -> Box<dyn EditorMetadata> {
        Box::new(self.clone())
    }
}

impl EditorView for ExampleEditorView {
    fn render(&self, _cx: &mut ViewContext<TabBar>) -> AnyElement {
        div()
            .text_color(rgb(0x555555))
            .child(format!("Example Editor View: {}", self.random_number))
            .size_full()
            .into_any()
    }
}