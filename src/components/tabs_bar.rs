use gpui::{
    actions, div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, 
    VisualContext, MouseButton, ElementId, Model, ModelContext, EventEmitter,
    Point, Pixels, px, FocusableView, View, AppContext, Context, prelude::FluentBuilder
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

use crate::components::editor_plugin::EditorView;
use crate::components::editors::level::{LevelEditor, LevelEditorView};
use gpui::AnyElement;

// Actions for keyboard shortcuts
actions!(tab_actions, [NextTab, PrevTab, CloseTab, NewTab, ReopenTab, MoveTabLeft, MoveTabRight]);

// Events emitted by the tab system
#[derive(Clone, Debug)]
pub struct TabChangeEvent {
    pub old_index: usize,
    pub new_index: usize,
}

#[derive(Clone, Debug)]
pub struct TabCloseEvent {
    pub index: usize,
    pub tab_id: String,
}

#[derive(Clone, Debug)]
pub struct TabReorderEvent {
    pub from_index: usize,
    pub to_index: usize,
}

// Core tab data structure
#[derive(Clone)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub is_pinned: bool,
    pub is_dirty: bool,
    pub is_preview: bool,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
}

impl Tab {
    pub fn new(id: String, title: String, content_type: String) -> Self {
        Self {
            id,
            title,
            icon: None,
            is_pinned: false,
            is_dirty: false,
            is_preview: false,
            content_type,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
    
    pub fn pinned(mut self) -> Self {
        self.is_pinned = true;
        self
    }
    
    pub fn dirty(mut self) -> Self {
        self.is_dirty = true;
        self
    }
    
    pub fn preview(mut self) -> Self {
        self.is_preview = true;
        self
    }
}

// Tab state management model
pub struct TabState {
    tabs: Vec<Tab>,
    active_tab: usize,
    tab_history: Vec<usize>,
    next_tab_id: AtomicUsize,
    show_close_buttons: bool,
    enable_drag_reorder: bool,
    max_tab_width: f32,
    min_tab_width: f32,
}

impl EventEmitter<TabChangeEvent> for TabState {}
impl EventEmitter<TabCloseEvent> for TabState {}
impl EventEmitter<TabReorderEvent> for TabState {}

impl TabState {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
            tab_history: Vec::new(),
            next_tab_id: AtomicUsize::new(0),
            show_close_buttons: true,
            enable_drag_reorder: true,
            max_tab_width: 200.0,
            min_tab_width: 120.0,
        }
    }
    
    pub fn add_tab(&mut self, mut tab: Tab, cx: &mut ModelContext<Self>) -> usize {
        // Generate unique ID if not provided
        if tab.id.is_empty() {
            tab.id = format!("tab_{}", self.next_tab_id.fetch_add(1, Ordering::Relaxed));
        }
        
        let index = self.tabs.len();
        self.tabs.push(tab);
        
        // Set as active tab and update history
        let old_active = self.active_tab;
        self.active_tab = index;
        self.tab_history.push(index);
        
        // Limit history size
        if self.tab_history.len() > 50 {
            self.tab_history.remove(0);
        }
        
        cx.emit(TabChangeEvent {
            old_index: old_active,
            new_index: index,
        });
        
        index
    }
    
    pub fn close_tab(&mut self, index: usize, cx: &mut ModelContext<Self>) -> bool {
        if index >= self.tabs.len() {
            return false;
        }
        
        let tab_id = self.tabs[index].id.clone();
        self.tabs.remove(index);
        
        // Update active tab
        if self.tabs.is_empty() {
            self.active_tab = 0;
        } else if self.active_tab >= index && self.active_tab > 0 {
            self.active_tab -= 1;
        } else if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
        
        // Update history
        self.tab_history.retain(|&i| i != index);
        for i in self.tab_history.iter_mut() {
            if *i > index {
                *i -= 1;
            }
        }
        
        cx.emit(TabCloseEvent { index, tab_id });
        true
    }
    
    pub fn switch_to_tab(&mut self, index: usize, cx: &mut ModelContext<Self>) -> bool {
        if index >= self.tabs.len() {
            return false;
        }
        
        let old_index = self.active_tab;
        self.active_tab = index;
        self.tab_history.push(index);
        
        // Limit history size
        if self.tab_history.len() > 50 {
            self.tab_history.remove(0);
        }
        
        cx.emit(TabChangeEvent {
            old_index,
            new_index: index,
        });
        
        true
    }
    
    pub fn move_tab(&mut self, from_index: usize, to_index: usize, cx: &mut ModelContext<Self>) -> bool {
        if from_index >= self.tabs.len() || to_index >= self.tabs.len() || from_index == to_index {
            return false;
        }
        
        let tab = self.tabs.remove(from_index);
        self.tabs.insert(to_index, tab);
        
        // Update active tab index
        if self.active_tab == from_index {
            self.active_tab = to_index;
        } else if from_index < self.active_tab && to_index >= self.active_tab {
            self.active_tab -= 1;
        } else if from_index > self.active_tab && to_index <= self.active_tab {
            self.active_tab += 1;
        }
        
        cx.emit(TabReorderEvent { from_index, to_index });
        true
    }
    
    pub fn next_tab(&mut self, cx: &mut ModelContext<Self>) {
        if !self.tabs.is_empty() {
            let next = (self.active_tab + 1) % self.tabs.len();
            self.switch_to_tab(next, cx);
        }
    }
    
    pub fn prev_tab(&mut self, cx: &mut ModelContext<Self>) {
        if !self.tabs.is_empty() {
            let prev = if self.active_tab > 0 {
                self.active_tab - 1
            } else {
                self.tabs.len() - 1
            };
            self.switch_to_tab(prev, cx);
        }
    }
    
    pub fn get_tabs(&self) -> &Vec<Tab> {
        &self.tabs
    }
    
    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab)
    }
    
    pub fn get_active_index(&self) -> usize {
        self.active_tab
    }
}

// Context menu for tabs
pub struct TabContextMenu {
    is_visible: bool,
    position: Point<Pixels>,
    target_tab: usize,
    tab_state: Model<TabState>,
}

impl TabContextMenu {
    pub fn new(tab_state: Model<TabState>) -> Self {
        Self {
            is_visible: false,
            position: Point::new(px(0.0), px(0.0)),
            target_tab: 0,
            tab_state,
        }
    }
    
    pub fn show_at(&mut self, position: Point<Pixels>, tab_index: usize) {
        self.is_visible = true;
        self.position = position;
        self.target_tab = tab_index;
    }
    
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

impl Render for TabContextMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }
        
        let target_tab = self.target_tab;
        let tab_state = self.tab_state.clone();
        
        div()
            .absolute()
            .inset_0()
            .on_mouse_down(MouseButton::Left, cx.listener(|this: &mut TabContextMenu, _event, _cx| {
                this.hide();
            }))
            .child(
                div()
                    .absolute()
                    .left(self.position.x)
                    .top(self.position.y)
                    .bg(rgb(0x2A2A2A))
                    .border_1()
                    .border_color(rgb(0x404040))
                    .rounded_md()
                    .shadow_lg()
                    .min_w(px(180.0))
                    .py_1()
                    .child(
                        {
                            let tab_state = tab_state.clone();
                            div()
                                .px_3()
                                .py_2()
                                .cursor_pointer()
                                .text_color(rgb(0xE0E0E0))
                                .hover(|style| style.bg(rgb(0x404040)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this: &mut TabContextMenu, _event, _cx| {
                                    tab_state.update(_cx, |state, cx| {
                                        state.close_tab(target_tab, cx);
                                    });
                                    this.hide();
                                }))
                                .child("Close Tab")
                        }
                    )
                    .child(
                        {
                            let tab_state = tab_state.clone();
                            div()
                                .px_3()
                                .py_2()
                                .cursor_pointer()
                                .text_color(rgb(0xE0E0E0))
                                .hover(|style| style.bg(rgb(0x404040)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this: &mut TabContextMenu, _event, _cx| {
                                    tab_state.update(_cx, |state, cx| {
                                        let mut indices_to_close: Vec<usize> = (0..state.tabs.len())
                                            .filter(|&i| i != target_tab)
                                            .collect();
                                        indices_to_close.reverse();
                                        
                                        for index in indices_to_close {
                                            state.close_tab(index, cx);
                                        }
                                    });
                                    this.hide();
                                }))
                                .child("Close Other Tabs")
                        }
                    )
                    .child(
                        {
                            let tab_state = tab_state.clone();
                            div()
                                .px_3()
                                .py_2()
                                .cursor_pointer()
                                .text_color(rgb(0xE0E0E0))
                                .hover(|style| style.bg(rgb(0x404040)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this: &mut TabContextMenu, _event, _cx| {
                                    tab_state.update(_cx, |state, _cx2| {
                                        if let Some(tab) = state.tabs.get_mut(target_tab) {
                                            tab.is_pinned = !tab.is_pinned;
                                        }
                                    });
                                    this.hide();
                                }))
                                .child("Pin Tab")
                        }
                    )
            )
    }
}

// Main tab bar component
pub struct TabBar {
    id: ElementId,
    tab_state: Model<TabState>,
    context_menu: View<TabContextMenu>,
    drag_state: Option<DragState>,
    focus_handle: gpui::FocusHandle,
}

struct DragState {
    dragging_tab: usize,
    drag_offset: Point<Pixels>,
    original_position: Point<Pixels>,
}

impl TabBar {
    pub fn new(id: ElementId, cx: &mut ViewContext<Self>) -> Self {
        let tab_state = cx.new_model(|_| TabState::new());
        let context_menu = cx.new_view(|_| TabContextMenu::new(tab_state.clone()));
        let focus_handle = cx.focus_handle();
        
        Self {
            id,
            tab_state,
            context_menu,
            drag_state: None,
            focus_handle,
        }
    }
    
    pub fn add_tab(&mut self, tab: Tab, cx: &mut ViewContext<Self>) -> usize {
        self.tab_state.update(cx, |state, cx| state.add_tab(tab, cx))
    }
    
    pub fn get_tab_state(&self) -> &Model<TabState> {
        &self.tab_state
    }
    
    fn render_tab(&self, index: usize, tab: &Tab, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_active = self.tab_state.read(cx).get_active_index() == index;
        let tab_state = self.tab_state.clone();
        let context_menu = self.context_menu.clone();
        let tab_state_for_tab = tab_state.clone();
        let tab_state_for_close = tab_state.clone();

        div()
            .flex()
            .items_center()
            .px_3()
            .py_2()
            .min_w(px(120.0))
            .max_w(px(200.0))
            .cursor_pointer()
            .border_b_2()
            .border_color(if is_active {
                rgb(0x2F80ED)
            } else {
                rgb(0x00000000) // Transparent
            })
            .bg(if is_active {
                rgb(0x1E1E1E)
            } else {
                rgb(0x141414)
            })
            .hover(|style| {
                if !is_active {
                    style.bg(rgb(0x1A1A1A))
                } else {
                    style
                }
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |_this: &mut TabBar, _event: &gpui::MouseDownEvent, cx| {
                tab_state_for_tab.update(cx, |state, cx| {
                    state.switch_to_tab(index, cx);
                });
            }))
            .on_mouse_down(MouseButton::Right, cx.listener(move |_this: &mut TabBar, event: &gpui::MouseDownEvent, cx| {
                context_menu.update(cx, |menu, _| {
                    menu.show_at(event.position, index);
                });
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .flex_1()
                    .min_w_0()
                    .when_some(tab.icon.as_ref(), |el, icon| {
                        el.child(
                            div()
                                .text_sm()
                                .child(icon.clone())
                        )
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(if is_active {
                                rgb(0xE0E0E0)
                            } else {
                                rgb(0xB0B0B0)
                            })
                            .child(tab.title.clone())
                    )
                    .when(tab.is_dirty, |el| {
                        el.child(
                            div()
                                .w_2()
                                .h_2()
                                .bg(rgb(0xFF6B6B))
                                .rounded_full()
                        )
                    })
            )
            .child(
                div()
                    .w_4()
                    .h_4()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_color(rgb(0x808080))
                    .hover(|style| {
                        style
                            .bg(rgb(0x404040))
                            .text_color(rgb(0xE0E0E0))
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener({
                        let tab_state = tab_state_for_close.clone();
                        move |_this: &mut TabBar, _event: &gpui::MouseDownEvent, cx| {
                            tab_state.update(cx, |state, cx| {
                                state.close_tab(index, cx);
                            });
                        }
                    }))
                    .child("×")
            )
    }

    fn render_new_tab_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tab_state = self.tab_state.clone();
        
        div()
            .flex()
            .items_center()
            .justify_center()
            .w_8()
            .h_8()
            .cursor_pointer()
            .text_color(rgb(0x808080))
            .rounded_sm()
            .hover(|style| {
                style
                    .bg(rgb(0x1A1A1A))
                    .text_color(rgb(0xE0E0E0))
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |_this: &mut TabBar, _event, cx| {
                let new_tab = Tab::new(
                    String::new(),
                    "New Tab".to_string(),
                    "editor".to_string(),
                );
                tab_state.update(cx, |state, cx| {
                    state.add_tab(new_tab, cx);
                });
            }))
            .child("+")
    }
}

impl FocusableView for TabBar {
    fn focus_handle(&self, _cx: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TabBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = self.tab_state.read(cx).get_tabs().clone();

        div()
            .key_context("TabBar")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &NextTab, cx| {
                this.tab_state.update(cx, |state, cx| state.next_tab(cx));
            }))
            .on_action(cx.listener(|this, _: &PrevTab, cx| {
                this.tab_state.update(cx, |state, cx| state.prev_tab(cx));
            }))
            .on_action(cx.listener(|this, _: &CloseTab, cx| {
                let active_index = this.tab_state.read(cx).get_active_index();
                this.tab_state.update(cx, |state, cx| {
                    state.close_tab(active_index, cx);
                });
            }))
            .on_action(cx.listener(|this, _: &NewTab, cx| {
                let new_tab = Tab::new(
                    String::new(),
                    "New Tab".to_string(),
                    "editor".to_string(),
                );
                this.tab_state.update(cx, |state, cx| {
                    state.add_tab(new_tab, cx);
                });
            }))
            .flex()
            .items_end()
            .bg(rgb(0x0A0A0A))
            .border_b_1()
            .border_color(rgb(0x2A2A2A))
            .child(
                div()
                    .flex()
                    .items_end()
                    .flex_1()
                    .children({
                        let mut children = Vec::new();
                        for (i, tab) in tabs.iter().enumerate() {
                            children.push(self.render_tab(i, tab, cx).into_element());
                        }
                        children
                    })
            )
            .child(self.render_new_tab_button(cx))
    }
}


// Complete tab system with content area
pub struct TabSystem {
    id: ElementId,
    tab_bar: View<TabBar>,
    content_renderers: HashMap<String, Box<dyn EditorView>>,
}

impl TabSystem {
    pub fn new(id: ElementId, cx: &mut ViewContext<Self>) -> Self {
        let tab_bar = cx.new_view(|cx| TabBar::new(ElementId::Name("tab_bar".into()), cx));
        
        Self {
            id,
            tab_bar,
            content_renderers: HashMap::new(),
        }
    }
    
    pub fn register_content_renderer<V: EditorView + 'static>(&mut self, content_type: String, view: V) {
        self.content_renderers.insert(content_type, Box::new(view));
    }
    
    pub fn add_tab(&mut self, tab: Tab, cx: &mut ViewContext<Self>) -> usize {
        self.tab_bar.update(cx, |bar, cx| bar.add_tab(tab, cx))
    }
    
    pub fn get_tab_state(&self, cx: &ViewContext<Self>) -> Model<TabState> {
        self.tab_bar.read(cx).get_tab_state().clone()
    }
    
    fn render_content(&self, cx: &mut ViewContext<Self>) -> gpui::Div {
        let tab_state = self.tab_bar.read(cx).get_tab_state().read(cx);

        let mut content_div = div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0x808080));

        if let Some(active_tab) = tab_state.get_active_tab() {
            if let Some(view) = self.content_renderers.get(&active_tab.content_type) {
                // Downcast cx to ViewContext<TabBar> for EditorView trait
                // SAFETY: This is safe because TabSystem always contains a TabBar as its main child
                let cx_ptr = cx as *mut ViewContext<Self> as *mut ViewContext<TabBar>;
                let cx_tabbar: &mut ViewContext<TabBar> = unsafe { &mut *cx_ptr };
                content_div = content_div.child(view.render(cx_tabbar));
            } else {
                content_div = content_div.child("No content available");
            }
        } else {
            content_div = content_div.child("No content available");
        }

        content_div

    }
}

impl Render for TabSystem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tab_bar = self.tab_bar.clone();
        let context_menu = self.tab_bar.read(cx).context_menu.clone();
        div()
            .size_full()
            .relative()
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .child(tab_bar)
                    .child(self.render_content(cx))
            )
            .child(
                context_menu
            )
    }
}

// Factory function for creating a TabSystem with a "Level Editor" tab and renderer
pub fn create_tab_system_with_level_editor(cx: &mut ViewContext<impl Render>) -> View<TabSystem> {
    cx.new_view(|cx| {
        let mut tab_system = TabSystem::new(ElementId::Name("main_tab_system".into()), cx);

        tab_system.register_content_renderer("level_editor".to_string(), LevelEditorView::new());

        // Add a LevelEditor tab on creation
        let tab = Tab::new(
            String::new(),
            "Level Editor".to_string(),
            "level_editor".to_string(),
        );

        let other_tab = Tab::new(
            String::new(),
            "Settings".to_string(),
            "settings".to_string(),
        );
        tab_system.add_tab(other_tab, cx);
        tab_system.add_tab(tab, cx);

        tab_system
    })
}