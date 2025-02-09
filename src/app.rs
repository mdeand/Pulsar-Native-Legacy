use gpui::{div, IntoElement, ParentElement, Render, ViewContext, VisualContext, WindowContext};

use crate::components::{
    title_bar::TitleBar,
    menu_bar::AppMenuBar
};


pub struct App {}

impl App {
    pub fn new(cx: &mut WindowContext) -> gpui::View<Self> {
        cx.new_view(|_| Self {})
    }
}

impl Render for App {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().children(vec![
            TitleBar::new(cx).into_any_element(),
            AppMenuBar::new(cx).into_any_element()
        ])
    }
}
