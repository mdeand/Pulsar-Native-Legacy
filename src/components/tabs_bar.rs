use gpui::{div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::app::App;

static SELECTED_TAB: AtomicUsize = AtomicUsize::new(0);

pub struct TabBar;

impl TabBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        cx.new_view(|_| Self)
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
            .child(
                div()
                    .flex()
                    .items_end()
                    .px_1()
                    .children(
                        tabs.iter().enumerate().map(|(index, tab_name)| {
                            let current_tab = SELECTED_TAB.load(Ordering::Relaxed);
                            let is_selected = index == current_tab;
                            let bg_color = if is_selected { 
                                rgb(0x141414) 
                            } else { 
                                rgb(0x080808) 
                            };

                            div()
                                .on_mouse_up(gpui::MouseButton::Left, move |_, cx| {
                                    SELECTED_TAB.store(index, Ordering::Relaxed);
                                    
                                    // Schedule a rerender of the window
                                    cx.refresh();
                                })
                                .flex()
                                .items_center()
                                .px_6()
                                .py_2()
                                .bg(bg_color)
                                .rounded_t_sm()
                                .text_color(if is_selected { rgb(0xE0E0E0) } else { rgb(0x808080) })
                                .text_sm()
                                .cursor_pointer()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .relative()
                                        .child(*tab_name)
                                        .child(
                                            if is_selected {
                                                div()
                                                    .absolute()
                                                    .bg(rgb(0x2F80ED))
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