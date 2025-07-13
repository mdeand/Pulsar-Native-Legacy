use gpui::{div, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext, WindowContext};

use crate::components::{
    title_bar::TitleBar,
    menu_bar::AppMenuBar,
    tabs_bar
};


pub struct App {}

impl App {
    pub fn new(cx: &mut WindowContext) -> gpui::View<Self> {
        cx.new_view(|_| Self {})
    }
}

impl Render for App {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .children(vec![
                TitleBar::new(cx).into_any_element(),
                AppMenuBar::new(cx).into_any_element(),
                tabs_bar::create_tab_system_with_level_editor(cx)
                    .into_any_element(),
            ])
    }
}
