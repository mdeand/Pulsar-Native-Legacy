use gpui::{div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, WindowContext, VisualContext, MouseButton};
use super::tab_registry::{get_all_editors, get_editor, register_editor};
use super::editor_plugin::{EditorMetadata, EditorView};
use std::sync::atomic::{AtomicUsize, Ordering};
use super::tab_instance::TabInstance;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::app::App;


use super::editors::level::LevelEditor;
use super::editors::example::ExampleEditor;


static NEXT_TAB_ID: AtomicUsize = AtomicUsize::new(0);
static SELECTED_TAB: AtomicUsize = AtomicUsize::new(0);
static SHOW_NEW_TAB_MENU: AtomicUsize = AtomicUsize::new(0);
static OPEN_TABS: Lazy<Mutex<Vec<TabInstance>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub struct TabBar;

impl TabBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        // Register default editors
        Self::register_default_editors();
        cx.new_view(|_| Self)
    }

    fn register_default_editors() {
        // Register built-in editors here
        register_editor(LevelEditor);
        register_editor(ExampleEditor);
        // register_editor(TerrainEditor);
        // register_editor(SceneEditor);
        // register_editor(AnimationEditor);
        // register_editor(ConsoleEditor);
        // register_editor(InspectorEditor);
    }

    pub fn add_tab(editor_name: &str) -> usize {
        let id = NEXT_TAB_ID.fetch_add(1, Ordering::Relaxed);
        let instance = TabInstance {
            id,
            editor_name: editor_name.to_string(),
            title: editor_name.to_string(),
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
            .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, cx: &mut WindowContext| {
                if show_menu {
                    SHOW_NEW_TAB_MENU.store(0, Ordering::Relaxed);
                    cx.refresh();
                }
            })
            .child(
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
                                                    .children(get_all_editors().iter().map(|editor| {
                                                        let editor_name = editor.name().to_string();
                                                        let editor_icon: String = editor.icon().to_string(); //TODO: use icon in tabs list to display in the tabe title slot
                                                        div()
                                                            .px_4()
                                                            .py_2()
                                                            .text_color(rgb(0xE0E0E0))
                                                            .rounded_sm()
                                                            .cursor_pointer()
                                                            .hover(|s| s.bg(rgb(0x2F80ED)))
                                                            .on_mouse_down(MouseButton::Left, move |_: &gpui::MouseDownEvent, cx: &mut WindowContext| {
                                                                TabBar::add_tab(&editor_name);
                                                                SHOW_NEW_TAB_MENU.store(0, Ordering::Relaxed);
                                                                cx.refresh();
                                                            })
                                                            .child(format!("{}", editor.title()))
                                                    }))
                                            )
                                    );
                                }

                                elements
                            })
                    )
            )
            .child(
                {
                    let current_tab = SELECTED_TAB.load(Ordering::Relaxed);
                    let current_tab_info = tabs.iter().find(|t| t.id == current_tab);
                    
                    div()
                        .flex_1()
                        .bg(rgb(0x141414))
                        .p_4()
                        .child(
                            if let Some(tab) = current_tab_info {
                                if let Some(editor) = get_editor(&tab.editor_name) {
                                    let instance = editor.create_view(cx);
                                    div().child(instance.render(cx))
                                } else {
                                    div()
                                        .text_color(rgb(0x808080))
                                        .child("Editor not found")
                                }
                            } else {
                                div()
                                    .text_color(rgb(0x808080))
                                    .child("No Tab Selected")
                            }
                        )
                }
            )
    }
}