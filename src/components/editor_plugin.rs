use gpui::{div, rgb, IntoElement, ViewContext};
use crate::app::App;
use crate::components::tabs_bar::TabBar;

pub trait EditorMetadata: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn icon(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn create_view(&self, cx: &mut ViewContext<TabBar>) -> impl EditorView;
}

pub trait EditorView: Send + Sync + 'static {
    fn render(&self, cx: &mut ViewContext<TabBar>) -> impl gpui::Element;
}