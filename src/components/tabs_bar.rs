use gpui::{div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, WindowContext, VisualContext, MouseButton};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::app::App;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum EditorType {
    Level,
    Terrain,
    Scene,
    Animation,
    Console,
    Inspector,
}

#[derive(Clone, Debug)]
struct TabInstance {
    id: usize,
    editor_type: EditorType,
    title: String,
}

static SELECTED_TAB: AtomicUsize = AtomicUsize::new(0);
static NEXT_TAB_ID: AtomicUsize = AtomicUsize::new(0);
static OPEN_TABS: Lazy<Mutex<Vec<TabInstance>>> = Lazy::new(|| Mutex::new(Vec::new()));
static SHOW_NEW_TAB_MENU: AtomicUsize = AtomicUsize::new(0); // 0 = hidden, 1 = shown

pub struct TabBar;

impl TabBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        cx.new_view(|_| Self)
    }

    pub fn add_tab(editor_type: EditorType) -> usize {
        let id = NEXT_TAB_ID.fetch_add(1, Ordering::Relaxed);
        let instance = TabInstance {
            id,
            editor_type,
            title: format!("{:?}", editor_type),
        };
        
        let mut tabs = OPEN_TABS.lock().unwrap();
        tabs.push(instance);
        SELECTED_TAB.store(id, Ordering::Relaxed);
        id
    }

    pub fn close_tab(tab_id: usize) {
        let mut tabs = OPEN_TABS.lock().unwrap();
        if let Some(pos) = tabs.iter().position(|t| t.id == tab_id) {
            tabs.remove(pos);
            let current = SELECTED_TAB.load(Ordering::Relaxed);
            if current == tab_id {
                let new_selected = if pos > 0 { tabs.get(pos - 1) } else { tabs.get(0) };
                if let Some(tab) = new_selected {
                    SELECTED_TAB.store(tab.id, Ordering::Relaxed);
                }
            }
        }
    }
}

impl Render for TabBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = OPEN_TABS.lock().unwrap();
        let show_menu = SHOW_NEW_TAB_MENU.load(Ordering::Relaxed) == 1;
        
        div()
            .size_full()
            .flex()
            .flex_col()
            // Add a click handler to the root element to close the menu when clicking outside
            .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, cx: &mut WindowContext| {
                if show_menu {
                    SHOW_NEW_TAB_MENU.store(0, Ordering::Relaxed);
                    cx.refresh();
                }
            })
            .child(
                // Tab headers and new tab button
                div()
                    .flex()
                    .h_12()
                    .items_end()
                    .bg(rgb(0x000000))
                    .child(
                        div()
                            .flex()
                            .items_end()
                            .px_1()
                            .children({
                                let mut elements: Vec<_> = tabs.iter().map(|tab| {
                                    let current_tab = SELECTED_TAB.load(Ordering::Relaxed);
                                    let is_selected = tab.id == current_tab;
                                    let bg_color = if is_selected { 
                                        rgb(0x141414) 
                                    } else { 
                                        rgb(0x080808) 
                                    };
                                    let tab_id = tab.id;

                                    div()
                                        .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, cx: &mut WindowContext| {
                                            SELECTED_TAB.store(tab_id, Ordering::Relaxed);
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
                                                .child(tab.title.to_string())
                                                .child(
                                                    if is_selected {
                                                        div()
                                                            .absolute()
                                                            .bottom_0()
                                                            .left_0()
                                                            .right_0()
                                                            .h_px()
                                                            .bg(rgb(0x2F80ED))
                                                    } else {
                                                        div()
                                                    }
                                                )
                                        )
                                }).collect();

                                // Add new tab button
                                elements.push(
                                    div()
                                        .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, cx: &mut WindowContext| {
                                            SHOW_NEW_TAB_MENU.store(1, Ordering::Relaxed);
                                            cx.refresh();
                                        })
                                        .flex()
                                        .items_center()
                                        .px_4()
                                        .py_2()
                                        .bg(rgb(0x080808))
                                        .rounded_t_sm()
                                        .text_color(rgb(0x808080))
                                        .text_sm()
                                        .cursor_pointer()
                                        .child("+")
                                );

                                // Add new tab menu if shown
                                if show_menu {
                                    elements.push(
                                        div()
                                            .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, _: &mut WindowContext| {})
                                            .absolute()
                                            .top_12()
                                            .right_0()
                                            .bg(rgb(0x1A1A1A))
                                            .rounded_sm()
                                            .shadow_lg()
                                            .p_2()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .children(vec![
                                                        EditorType::Level,
                                                        EditorType::Terrain,
                                                        EditorType::Scene,
                                                        EditorType::Animation,
                                                        EditorType::Console,
                                                        EditorType::Inspector,
                                                    ].iter().map(|editor_type| {
                                                        let editor_type = *editor_type;
                                                        div()
                                                            .px_4()
                                                            .py_2()
                                                            .text_color(rgb(0xE0E0E0))
                                                            .rounded_sm()
                                                            .cursor_pointer()
                                                            .hover(|s| s.bg(rgb(0x2F80ED)))
                                                            .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, cx: &mut WindowContext| {
                                                                TabBar::add_tab(editor_type);
                                                                SHOW_NEW_TAB_MENU.store(0, Ordering::Relaxed);
                                                                cx.refresh();
                                                            })
                                                            .child(format!("{:?}", editor_type))
                                                    }))
                                            )
                                    );
                                }

                                elements
                            })
                    )
            )
            .child(
                // Tab content
                {
                    let current_tab = SELECTED_TAB.load(Ordering::Relaxed);
                    let current_tab_info = tabs.iter().find(|t| t.id == current_tab);
                    
                    div()
                        .flex_1()
                        .bg(rgb(0x141414))
                        .p_4()
                        .child(
                            match current_tab_info.map(|t| &t.editor_type) {
                                Some(EditorType::Level) => div()
                                    .text_color(rgb(0xE0E0E0))
                                    .child("Level Editor Content")
                                    .size_full(),
                                Some(EditorType::Terrain) => div()
                                    .text_color(rgb(0xE0E0E0))
                                    .child("Terrain Editor Content")
                                    .size_full(),
                                Some(EditorType::Scene) => div()
                                    .text_color(rgb(0xE0E0E0))
                                    .child("Scene View Content")
                                    .size_full(),
                                Some(EditorType::Animation) => div()
                                    .text_color(rgb(0xE0E0E0))
                                    .child("Animation Timeline")
                                    .size_full(),
                                Some(EditorType::Console) => div()
                                    .text_color(rgb(0xE0E0E0))
                                    .bg(rgb(0x0A0A0A))
                                    .size_full()
                                    .p_2()
                                    .child("Console Output"),
                                Some(EditorType::Inspector) => div()
                                    .text_color(rgb(0xE0E0E0))
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .size_full()
                                    .children(vec![
                                        div().child("Properties"),
                                        div().child("Components"),
                                    ]),
                                None => div()
                                    .text_color(rgb(0x808080))
                                    .child("No Tab Selected")
                            }
                        )
                }
            )
    }
}