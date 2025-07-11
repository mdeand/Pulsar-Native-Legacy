use gpui::{canvas, div, rgb, fill, AnyElement, Element, ParentElement, Pixels, Styled, ViewContext};
use super::super::editor_plugin::{EditorMetadata, EditorView};
use crate::components::tabs_bar::TabBar;

#[derive(Clone)]
pub struct AsteriskEditor;
pub struct AsteriskEditorView {
    random_number: u32,
}

impl EditorMetadata for AsteriskEditor {
    fn name(&self)        -> &'static str { "Asterisk" }
    fn icon(&self)        -> &'static str { "🗺️" }
    fn title(&self)       -> &'static str { "Asterisk's editor" }
    fn description(&self) -> &'static str { "Editor for showing how editor tabs are created." }
    
    fn create_view(&self, _cx: &mut ViewContext<TabBar>) -> Box<(dyn EditorView + 'static)> {
        Box::new(AsteriskEditorView {
            random_number: rand::random::<u32>() % 1000,
        })
    }
    
    fn clone_box(&self) -> Box<dyn EditorMetadata> {
        Box::new(self.clone())
    }
}

impl EditorView for AsteriskEditorView {
    fn render(&self, _cx: &mut ViewContext<TabBar>) -> AnyElement {
        div()
            .text_color(rgb(0x555555))
            .child(format!("Asterisk Editor View: {}", self.random_number))
            .child(format!("Hello, world!"))
            .child(canvas(|_bounds, _cx| {
                // Prepaint logic here
                }, |bounds, _prepaint, cx| {
                    println!("Canvas bounds: {:?}", bounds);
                    bounds.center();
                })
                .w_full()
                .h_full()
            )
            .child(div()
                .text_color(rgb(0xffffff))
                .child("Asterisk Editor View")
                .size_full()
                .h_full()
            )
            .size_full()
            .h_full()
            .w_full()
            .into_any()
    }
}