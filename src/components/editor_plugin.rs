use gpui::{ViewContext, AnyElement};
use crate::components::tabs_bar::TabBar;

pub trait EditorMetadata: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn icon(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn create_view(&self, cx: &mut ViewContext<TabBar>) -> Box<dyn EditorView>;
    fn clone_box(&self) -> Box<dyn EditorMetadata>;
}

impl Clone for Box<dyn EditorMetadata> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait EditorView: Send + Sync + 'static {
    fn render(&self, cx: &mut ViewContext<TabBar>) -> AnyElement;
}