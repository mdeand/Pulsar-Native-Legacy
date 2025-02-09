use gpui::{div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};

use crate::app::App;

pub struct TabBar {
    selected_tab: usize,
}

impl TabBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        cx.new_view(|_| Self { 
            selected_tab: 0 
        })
    }

    pub fn selected_tab(&self) -> usize {
        self.selected_tab
    }

    pub fn set_selected_tab(&mut self, index: usize) {
        self.selected_tab = index;
    }
}

impl Render for TabBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = vec![
            "Scene", 
            "Game", 
            "Inspector",
            "Project",
            "Console",
            "Animation",
            "Profiler",
        ];

        div()
            .size_full()
            .flex()
            .h_12()
            .items_end()
            .bg(rgb(0x000000))
            // .style("box-shadow", "inset 0 0 20px rgba(0, 0, 0, 0.8)")
            .child(
                div()
                    .flex()
                    .items_end()
                    // .gap_0_5()
                    .px_1()
                    .children(
                        tabs.iter().enumerate().map(|(index, tab_name)| {
                            let is_selected = index == self.selected_tab;
                            let bg_color = if is_selected { 
                                rgb(0x141414) 
                            } else { 
                                rgb(0x080808) 
                            };
                            let border_color = if is_selected {
                                rgb(0x1A1A1A)
                            } else {
                                rgb(0x0A0A0A)
                            };

                            div()
                                .on_mouse_up(gpui::MouseButton::Left, move |event, context| {
                                    println!("Tab {} clicked", index);

                                    //self.set_selected_tab(index);
                                })
                                .flex()
                                .items_center()
                                .px_6()
                                .py_2()
                                .bg(bg_color)
                                // .border_x(border_color)
                                // .border_t(border_color)
                                .rounded_t_sm()
                                .text_color(if is_selected { rgb(0xE0E0E0) } else { rgb(0x808080) })
                                .text_sm()
                                .cursor_pointer()
                                // .style("transition", "all 0.15s ease")
                                // .style("box-shadow", if is_selected {
                                //     "inset 0 1px 0 rgba(255, 255, 255, 0.05)"
                                // } else {
                                //     "none"
                                // })
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .relative()
                                        .child("tab_name")
                                        .child(
                                            if is_selected {
                                                div()
                                                    .absolute()
                                                    // .left_n2()
                                                    // .right_n2()
                                                    // .h_0_5()
                                                    .bg(rgb(0x2F80ED))
                                                    // .style("box-shadow", "0 0 10px rgba(47, 128, 237, 0.3)")
                                            } else {
                                                div()
                                            }
                                        )
                                )
                        })
                    )
            )
    }
}