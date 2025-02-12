use gpui::{div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::app::App;

// Each tab has a type and a unique instance ID
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TabType {
    Scene,
    LevelEditor,
    TerrainEditor,
    Inspector,
    Console,
    Profiler,
}

impl TabType {
    fn display_name(&self) -> &'static str {
        match self {
            TabType::Scene => "Scene",
            TabType::LevelEditor => "Level Editor",
            TabType::TerrainEditor => "Terrain Editor",
            TabType::Inspector => "Inspector",
            TabType::Console => "Console",
            TabType::Profiler => "Profiler",
        }
    }
}

#[derive(Clone, Debug)]
struct TabInstance {
    tab_type: TabType,
    instance_id: usize,
    title: String,  // Can be customized per instance
}

static SELECTED_TAB: AtomicUsize = AtomicUsize::new(0);
static NEXT_INSTANCE_ID: AtomicUsize = AtomicUsize::new(0);

static OPEN_TABS: Lazy<Mutex<Vec<TabInstance>>> = Lazy::new(|| {
    let initial_tabs = vec![
        TabInstance {
            tab_type: TabType::Scene,
            instance_id: 0,
            title: "Scene".to_string(),
        },
        TabInstance {
            tab_type: TabType::Console,
            instance_id: 1,
            title: "Console".to_string(),
        },
    ];
    NEXT_INSTANCE_ID.store(2, Ordering::Relaxed);
    Mutex::new(initial_tabs)
});

pub struct TabBar;

impl TabBar {
    pub fn new(cx: &mut ViewContext<App>) -> gpui::View<Self> {
        cx.new_view(|_| Self)
    }

    // Helper functions for managing tabs
    pub fn add_tab(tab_type: TabType, title: Option<String>) -> usize {
        let instance_id = NEXT_INSTANCE_ID.fetch_add(1, Ordering::Relaxed);
        let new_tab = TabInstance {
            tab_type,
            instance_id,
            title: title.unwrap_or_else(|| tab_type.display_name().to_string()),
        };
        
        if let Ok(mut tabs) = OPEN_TABS.lock() {
            tabs.push(new_tab);
        }
        
        instance_id
    }

    pub fn close_tab(instance_id: usize) {
        if let Ok(mut tabs) = OPEN_TABS.lock() {
            if let Some(pos) = tabs.iter().position(|t| t.instance_id == instance_id) {
                tabs.remove(pos);
                // If we closed the selected tab, select the last tab
                let current_selected = SELECTED_TAB.load(Ordering::Relaxed);
                if current_selected >= tabs.len() {
                    SELECTED_TAB.store(tabs.len().saturating_sub(1), Ordering::Relaxed);
                }
            }
        }
    }
}

impl Render for TabBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = OPEN_TABS.lock().unwrap();
        let tabs = tabs.clone(); // Clone to release the lock
        
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Tab headers
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
                            .children(
                                tabs.iter().enumerate().map(|(index, tab)| {
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
                                            cx.window().update_window(|_| {});
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
                                                .gap_2()
                                                .items_center()
                                                .child(&tab.title)
                                                .child(
                                                    // Close button
                                                    div()
                                                        .px_2()
                                                        .on_mouse_up(gpui::MouseButton::Left, move |event, cx| {
                                                            event.stop_propagation();
                                                            TabBar::close_tab(tab.instance_id);
                                                            cx.window().update_window(|_| {});
                                                        })
                                                        .child("×")
                                                )
                                        )
                                })
                            )
                    )
            )
            .child(
                // Tab content
                {
                    let current_tab = SELECTED_TAB.load(Ordering::Relaxed);
                    if let Some(tab) = tabs.get(current_tab) {
                        div()
                            .flex_1()
                            .bg(rgb(0x141414))
                            .p_4()
                            .child(
                                match tab.tab_type {
                                    TabType::Scene => div()
                                        .text_color(rgb(0xE0E0E0))
                                        .child(format!("Scene View {}", tab.instance_id))
                                        .size_full(),
                                    TabType::LevelEditor => div()
                                        .text_color(rgb(0xE0E0E0))
                                        .child(format!("Level Editor {}", tab.instance_id))
                                        .size_full(),
                                    TabType::TerrainEditor => div()
                                        .text_color(rgb(0xE0E0E0))
                                        .child(format!("Terrain Editor {}", tab.instance_id))
                                        .size_full(),
                                    TabType::Inspector => div()
                                        .text_color(rgb(0xE0E0E0))
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .size_full()
                                        .children(vec![
                                            div().child("Transform"),
                                            div().child("Material"),
                                            div().child("Mesh Renderer"),
                                        ]),
                                    TabType::Console => div()
                                        .text_color(rgb(0xE0E0E0))
                                        .bg(rgb(0x0A0A0A))
                                        .size_full()
                                        .p_2()
                                        .child(format!("Console {}", tab.instance_id)),
                                    TabType::Profiler => div()
                                        .text_color(rgb(0xE0E0E0))
                                        .size_full()
                                        .child(format!("Profiler {}", tab.instance_id)),
                                }
                            )
                    } else {
                        div()
                    }
                }
            )
    }
}