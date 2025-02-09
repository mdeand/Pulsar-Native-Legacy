use gpui::{div, rgb, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};

use crate::app::App;

pub struct AppMenuBar {}

impl AppMenuBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        cx.new_view(|_| Self {})
    }
}

impl Render for AppMenuBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .h_8()
            .items_center()
            .px_4()
            .bg(rgb(0x0A0A0A))
            .child(
                div()
                    .flex()
                    .gap_6()
                    .text_color(rgb(0xCCCCCC))
                    .text_sm()
                    .children(vec![
                        div()
                            .cursor_pointer()
                            .child("File"),

                            // TODO - Add the correct styles to make this dropdown work properly
                            //.children(vec![
                            //    div()
                            //        .cursor_pointer()
                            //        .child("New"),
                            //    div()
                            //        .cursor_pointer()
                            //        .child("Open"),
                            //    div()
                            //        .cursor_pointer()
                            //        .child("Save"),
                            //    div()
                            //        .cursor_pointer()
                            //        .child("Save As"),
                            //    div()
                            //        .cursor_pointer()
                            //        .child("Close"),
                            //]),
                        div()
                            .cursor_pointer()
                            .child("Edit"),
                        div()
                            .cursor_pointer()
                            .child("View"),
                        div()
                            .cursor_pointer()
                            .child("Go"),
                        div()
                            .cursor_pointer()
                            .child("Run"),
                        div()
                            .cursor_pointer()
                            .child("Terminal"),
                        div()
                            .cursor_pointer()
                            .child("Window"),
                        div()
                            .cursor_pointer()
                            .child("Help"),
                    ])
            )
    }
}