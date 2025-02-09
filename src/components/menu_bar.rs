use gpui::{div, rgb, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};

use crate::app::App;

pub struct MenuBar {}

impl MenuBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        cx.new_view(|_| Self {  })
    }
}

impl Render for MenuBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
        .size_full()
        .flex()
        .h_10()
        .items_center()
        .justify_between()
        .px_4()
        .bg(rgb(0x0A0A0A))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_color(rgb(0x2F80ED))
                .child("◆")
                .child(
                    div()
                        .text_color(rgb(0x2F80ED))
                        .text_sm()
                        .child("Pulsar Engine")
                )
        )
        .child(
            div()
                .flex()
                .gap_4()
                .child(div().text_lg().text_color(rgb(0x666666)).child("−"))
                .child(div().text_lg().text_color(rgb(0x666666)).child("□"))
                .child(div().text_lg().text_color(rgb(0x666666)).child("×"))
        )
    }
}