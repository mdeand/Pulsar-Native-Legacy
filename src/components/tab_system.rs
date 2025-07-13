use gpui::{
    actions, div, rgb, InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext, MouseButton, ElementId, Model, ModelContext, EventEmitter,
    Point, Pixels, px, FocusableView, View, AppContext, Context, prelude::FluentBuilder,
    AnyElement, WindowContext
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// --- Actions, same as before ---
actions!(tab_actions, [NextTab, PrevTab, CloseTab, NewTab, ReopenTab, MoveTabLeft, MoveTabRight]);

// --- Global State ---
// We add a new state atom to track the visibility of the "new tab" dropdown.
static NEXT_TAB_ID: AtomicUsize = AtomicUsize::new(0);
static SELECTED_TAB: AtomicUsize = AtomicUsize::new(0);
static SHOW_CONTEXT_MENU: AtomicUsize = AtomicUsize::new(0);
static CONTEXT_MENU_POS: Lazy<Mutex<(f32, f32)>> = Lazy::new(|| Mutex::new((0.0, 0.0)));
static CONTEXT_MENU_TAB: AtomicUsize = AtomicUsize::new(0);
static OPEN_TABS: Lazy<Mutex<Vec<TabData>>> = Lazy::new(|| Mutex::new(Vec::new()));
static CLOSED_TABS: Lazy<Mutex<Vec<TabData>>> = Lazy::new(|| Mutex::new(Vec::new()));
static SHOW_NEW_TAB_DROPDOWN: AtomicUsize = AtomicUsize::new(0); // New state for dropdown

// --- Core Tab Traits and Registry ---

/// A trait for any content that can be displayed within a tab.
pub trait TabContentProvider: Send + Sync + 'static {
    fn render_content(&self, tab_id: usize) -> AnyElement;
    fn get_title(&self) -> String;
    fn is_dirty(&self) -> bool;
    fn can_close(&self) -> bool;
}

/// A new trait for types that can be registered and created from the UI.
/// Any struct implementing this trait can be added to the "New Tab" dropdown.
pub trait RegisterableTab: Send + Sync + 'static {
    /// The name to display in the "New Tab" dropdown menu.
    fn name(&self) -> &'static str;
    /// A function that creates a new instance of the tab's content provider.
    fn create(&self) -> Arc<dyn TabContentProvider>;
    /// An optional icon for the tab type.
    fn icon(&self) -> Option<String> { None }
}

// Global registry for all available tab types.
static TAB_TYPE_REGISTRY: Lazy<Mutex<Vec<Arc<dyn RegisterableTab>>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Public function to register a new tab type.
/// Call this from your application's setup code for each tab type you want to make available.
pub fn register_tab_type(tab_type: Arc<dyn RegisterableTab>) {
    TAB_TYPE_REGISTRY.lock().unwrap().push(tab_type);
}


// --- Default Content Provider ---
// This remains largely the same, but we'll create a corresponding `RegisterableTab` impl.
#[derive(Clone)]
pub struct TextContentProvider {
    title: String,
    content: String,
    is_dirty: bool,
}

impl TextContentProvider {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            is_dirty: false,
        }
    }
}

impl TabContentProvider for TextContentProvider {
    fn render_content(&self, _tab_id: usize) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .p_4()
            .child(
                div()
                    .text_lg()
                    .mb_4()
                    .text_color(rgb(0xE0E0E0))
                    .child(self.title.clone())
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xB0B0B0))
                    .child(self.content.clone())
            )
            .into_any_element()
    }
    
    fn get_title(&self) -> String {
        self.title.clone()
    }
    
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    
    fn can_close(&self) -> bool {
        true
    }
}

// --- Tab Data Structure ---
// This remains the same.
#[derive(Clone)]
pub struct TabData {
    pub id: usize,
    pub content_provider: Arc<dyn TabContentProvider>,
    pub icon: Option<String>,
    pub is_pinned: bool,
    pub is_preview: bool,
    pub metadata: HashMap<String, String>,
}

impl TabData {
    pub fn new(content_provider: Arc<dyn TabContentProvider>) -> Self {
        let id = NEXT_TAB_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            content_provider,
            icon: None,
            is_pinned: false,
            is_preview: false,
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
    
    pub fn get_title(&self) -> String {
        self.content_provider.get_title()
    }
    
    pub fn is_dirty(&self) -> bool {
        self.content_provider.is_dirty()
    }
    
    pub fn can_close(&self) -> bool {
        self.content_provider.can_close()
    }
}

// --- Tab System Implementation ---
pub struct TabSystem {
    focus_handle: gpui::FocusHandle,
}

impl TabSystem {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        cx.focus(&focus_handle);
        
        // We no longer add sample tabs here. They should be registered and added externally.
        
        Self {
            focus_handle,
        }
    }
    
    // All tab management functions (add, close, reopen, etc.) remain the same.
    pub fn add_tab(tab: TabData) -> usize {
        let mut tabs = OPEN_TABS.lock().unwrap();
        let tab_id = tab.id;
        tabs.push(tab);
        SELECTED_TAB.store(tab_id, Ordering::Relaxed);
        tab_id
    }
    
    pub fn close_tab(tab_id: usize) {
        let mut tabs = OPEN_TABS.lock().unwrap();
        if let Some(pos) = tabs.iter().position(|t| t.id == tab_id) {
            if !tabs[pos].can_close() { return; }
            let tab = tabs.remove(pos);
            
            let mut closed_tabs = CLOSED_TABS.lock().unwrap();
            closed_tabs.push(tab);
            if closed_tabs.len() > 10 {
                closed_tabs.remove(0);
            }
            drop(closed_tabs);
            
            let current = SELECTED_TAB.load(Ordering::Relaxed);
            if current == tab_id {
                let new_selected = if pos > 0 { tabs.get(pos - 1) } else { tabs.get(0) };
                if let Some(tab) = new_selected {
                    SELECTED_TAB.store(tab.id, Ordering::Relaxed);
                } else {
                    SELECTED_TAB.store(0, Ordering::Relaxed);
                }
            }
        }
    }
    
    pub fn reopen_last_closed_tab() {
        let mut closed_tabs = CLOSED_TABS.lock().unwrap();
        if let Some(tab) = closed_tabs.pop() {
            drop(closed_tabs);
            let tab_id = tab.id;
            let mut tabs = OPEN_TABS.lock().unwrap();
            tabs.push(tab);
            SELECTED_TAB.store(tab_id, Ordering::Relaxed);
        }
    }

    pub fn close_other_tabs(keep_tab_id: usize) {
        let mut tabs = OPEN_TABS.lock().unwrap();
        let mut closed_tabs = CLOSED_TABS.lock().unwrap();
        
        tabs.retain(|tab| {
            if tab.id != keep_tab_id && tab.can_close() {
                closed_tabs.push(tab.clone());
                false
            } else {
                true
            }
        });
        
        while closed_tabs.len() > 10 {
            closed_tabs.remove(0);
        }
        
        SELECTED_TAB.store(keep_tab_id, Ordering::Relaxed);
    }
    
    pub fn toggle_pin_tab(tab_id: usize) {
        let mut tabs = OPEN_TABS.lock().unwrap();
        if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.is_pinned = !tab.is_pinned;
        }
    }
    
    pub fn next_tab() {
        let tabs = OPEN_TABS.lock().unwrap();
        if !tabs.is_empty() {
            let current = SELECTED_TAB.load(Ordering::Relaxed);
            if let Some(pos) = tabs.iter().position(|t| t.id == current) {
                let next_pos = (pos + 1) % tabs.len();
                SELECTED_TAB.store(tabs[next_pos].id, Ordering::Relaxed);
            } else if let Some(first_tab) = tabs.first() {
                SELECTED_TAB.store(first_tab.id, Ordering::Relaxed);
            }
        }
    }
    
    pub fn prev_tab() {
        let tabs = OPEN_TABS.lock().unwrap();
        if !tabs.is_empty() {
            let current = SELECTED_TAB.load(Ordering::Relaxed);
            if let Some(pos) = tabs.iter().position(|t| t.id == current) {
                let prev_pos = if pos > 0 { pos - 1 } else { tabs.len() - 1 };
                SELECTED_TAB.store(tabs[prev_pos].id, Ordering::Relaxed);
            } else if let Some(first_tab) = tabs.first() {
                SELECTED_TAB.store(first_tab.id, Ordering::Relaxed);
            }
        }
    }
    
    // --- UI Rendering Methods ---

    fn render_context_menu(&self, _cx: &mut ViewContext<Self>) -> AnyElement {
        if SHOW_CONTEXT_MENU.load(Ordering::Relaxed) == 0 {
            return div().into_any_element();
        }
        
        let menu_pos = CONTEXT_MENU_POS.lock().unwrap();
        let (x, y) = *menu_pos;
        
        let target_tab_id = CONTEXT_MENU_TAB.load(Ordering::Relaxed);
        let tabs = OPEN_TABS.lock().unwrap();
        let target_tab = tabs.iter().find(|t| t.id == target_tab_id);
        let can_close = target_tab.map(|t| t.can_close()).unwrap_or(false);
        let is_pinned = target_tab.map(|t| t.is_pinned).unwrap_or(false);
        let has_other_tabs = tabs.len() > 1;
        
        div()
            .absolute()
            .left(px(x))
            .top(px(y))
            .bg(rgb(0x2A2A2A))
            .border_1()
            .border_color(rgb(0x404040))
            .rounded_md()
            .shadow_lg()
            .min_w(px(180.0))
            .py_1()
            .child(
                div()
                    .px_3().py_2().cursor_pointer()
                    .text_color(if can_close { rgb(0xE0E0E0) } else { rgb(0x666666) })
                    .when(can_close, |el| {
                        el.hover(|style| style.bg(rgb(0x404040)))
                            .on_mouse_down(MouseButton::Left, move |_, cx| {
                                Self::close_tab(target_tab_id);
                                SHOW_CONTEXT_MENU.store(0, Ordering::Relaxed);
                                cx.refresh();
                            })
                    })
                    .child("Close Tab")
            )
            .child(
                div()
                    .px_3().py_2().cursor_pointer()
                    .text_color(if has_other_tabs { rgb(0xE0E0E0) } else { rgb(0x666666) })
                    .when(has_other_tabs, |el| {
                        el.hover(|style| style.bg(rgb(0x404040)))
                            .on_mouse_down(MouseButton::Left, move |_, cx| {
                                Self::close_other_tabs(target_tab_id);
                                SHOW_CONTEXT_MENU.store(0, Ordering::Relaxed);
                                cx.refresh();
                            })
                    })
                    .child("Close Other Tabs")
            )
            .child(div().h(px(1.0)).bg(rgb(0x404040)).mx_1().my_1())
            .child(
                div()
                    .px_3().py_2().cursor_pointer().text_color(rgb(0xE0E0E0))
                    .hover(|style| style.bg(rgb(0x404040)))
                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                        Self::toggle_pin_tab(target_tab_id);
                        SHOW_CONTEXT_MENU.store(0, Ordering::Relaxed);
                        cx.refresh();
                    })
                    .child(if is_pinned { "Unpin Tab" } else { "Pin Tab" })
            )
            .into_any_element()
    }
    
    /// New function to render the dropdown for creating new tabs.
    fn render_new_tab_dropdown(&self, _cx: &mut ViewContext<Self>) -> AnyElement {
        if SHOW_NEW_TAB_DROPDOWN.load(Ordering::Relaxed) == 0 {
            return div().into_any_element();
        }

        let registry = TAB_TYPE_REGISTRY.lock().unwrap();
        if registry.is_empty() {
            return div().into_any_element(); // Don't show if no types are registered
        }

        div()
            .absolute()
            .top(px(38.0)) // Position below the tab bar
            .right(px(40.0)) // Align near the '+' and '↶' buttons
            .bg(rgb(0x2A2A2A))
            .border_1()
            .border_color(rgb(0x404040))
            .rounded_md()
            .shadow_lg()
            .min_w(px(180.0))
            .py_1()
            .children(registry.iter().map(|tab_type| {
                let tab_type_clone = tab_type.clone();
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .cursor_pointer()
                    .text_color(rgb(0xE0E0E0))
                    .hover(|style| style.bg(rgb(0x404040)))
                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                        let content_provider = tab_type_clone.create();
                        let mut new_tab = TabData::new(content_provider);
                        if let Some(icon) = tab_type_clone.icon() {
                            new_tab = new_tab.with_icon(icon);
                        }
                        Self::add_tab(new_tab);
                        SHOW_NEW_TAB_DROPDOWN.store(0, Ordering::Relaxed);
                        cx.refresh();
                    })
                    .when_some(tab_type.icon(), |el, icon| {
                        el.child(div().child(icon))
                    })
                    .child(div().child(tab_type.name()))
            }))
            .into_any_element()
    }

    fn render_tab_bar(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = OPEN_TABS.lock().unwrap();
        let selected_tab_id = SELECTED_TAB.load(Ordering::Relaxed);
        let has_closed_tabs = !CLOSED_TABS.lock().unwrap().is_empty();
        
        div()
            .flex()
            .h(px(40.0))
            .items_end()
            .bg(rgb(0x0A0A0A))
            .border_b_1()
            .border_color(rgb(0x2A2A2A))
            .child(
                div()
                    .flex()
                    .items_end()
                    .flex_1()
                    .px_1()
                    .children(tabs.iter().map(|tab| {
                        let is_selected = tab.id == selected_tab_id;
                        let tab_id = tab.id;
                        
                        let tab_bg_color = if is_selected { rgb(0x1E1E1E) } else if tab.is_pinned { rgb(0x1A1A1A) } else { rgb(0x141414) };
                        let border_color = if is_selected { rgb(0x2F80ED) } else if tab.is_pinned { rgb(0x666666) } else { rgb(0x00000000) };

                        div()
                            .flex().items_center().px_3().py_2().min_w(px(120.0)).max_w(px(200.0))
                            .cursor_pointer().border_b_2().border_color(border_color).bg(tab_bg_color)
                            .hover(|style| if !is_selected { style.bg(rgb(0x1A1A1A)) } else { style })
                            .on_mouse_down(MouseButton::Left, move |_, cx| {
                                SELECTED_TAB.store(tab_id, Ordering::Relaxed);
                                SHOW_NEW_TAB_DROPDOWN.store(0, Ordering::Relaxed);
                                cx.refresh();
                            })
                            .on_mouse_down(MouseButton::Right, move |event, cx| {
                                CONTEXT_MENU_TAB.store(tab_id, Ordering::Relaxed);
                                *CONTEXT_MENU_POS.lock().unwrap() = (event.position.x.0, event.position.y.0);
                                SHOW_CONTEXT_MENU.store(1, Ordering::Relaxed);
                                cx.refresh();
                            })
                            .child(
                                div()
                                    .flex().items_center().gap_2().flex_1().min_w_0()
                                    .when_some(tab.icon.as_ref(), |el, icon| el.child(div().text_sm().text_color(rgb(0xB0B0B0)).child(icon.clone())))
                                    .when(tab.is_pinned, |el| el.child(div().text_xs().text_color(rgb(0x666666)).child("📌")))
                                    .child(
                                        div().flex_1().text_sm().text_color(if is_selected { rgb(0xE0E0E0) } else { rgb(0xB0B0B0) })
                                            .overflow_hidden().whitespace_nowrap().child(tab.get_title())
                                    )
                                    .when(tab.is_dirty(), |el| el.child(div().w_2().h_2().bg(rgb(0xFF6B6B)).rounded_full()))
                            )
                            .when(tab.can_close() && !tab.is_pinned, |el| {
                                el.child(
                                    div()
                                        .w_4().h_4().flex().items_center().justify_center().rounded_sm().cursor_pointer()
                                        .text_color(rgb(0x808080))
                                        .hover(|style| style.bg(rgb(0x404040)).text_color(rgb(0xE0E0E0)))
                                        .on_mouse_down(MouseButton::Left, move |_, cx| {
                                            Self::close_tab(tab_id);
                                            cx.refresh();
                                        })
                                        .child("×")
                                )
                            })
                    }))
            )
            .child(
                div() // Container for action buttons
                    .flex()
                    .items_center()
                    .h_full()
                    .px_2()
                    .child(
                        // The "New Tab" button now toggles the dropdown.
                        div()
                            .flex().items_center().justify_center().w_8().h_8().cursor_pointer()
                            .text_color(rgb(0x808080)).rounded_sm()
                            .hover(|style| style.bg(rgb(0x1A1A1A)).text_color(rgb(0xE0E0E0)))
                            .on_mouse_down(MouseButton::Left, move |_, cx| {
                                let current_state = SHOW_NEW_TAB_DROPDOWN.load(Ordering::Relaxed);
                                SHOW_NEW_TAB_DROPDOWN.store(1 - current_state, Ordering::Relaxed);
                                cx.refresh();
                            })
                            .child("+")
                    )
                    .when(has_closed_tabs, |el| {
                        el.child(
                            // Reopen tab button remains the same.
                            div()
                                .flex().items_center().justify_center().w_8().h_8().cursor_pointer()
                                .text_color(rgb(0x808080)).rounded_sm()
                                .hover(|style| style.bg(rgb(0x1A1A1A)).text_color(rgb(0xE0E0E0)))
                                .on_mouse_down(MouseButton::Left, move |_, cx| {
                                    Self::reopen_last_closed_tab();
                                    cx.refresh();
                                })
                                .child("↶")
                        )
                    })
            )
    }
    
    fn render_content(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let tabs = OPEN_TABS.lock().unwrap();
        let selected_tab_id = SELECTED_TAB.load(Ordering::Relaxed);
        
        if let Some(active_tab) = tabs.iter().find(|t| t.id == selected_tab_id) {
            div()
                .flex_1().flex().overflow_hidden().bg(rgb(0x1A1A1A))
                .child(active_tab.content_provider.render_content(active_tab.id))
        } else {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .bg(rgb(0x18181A))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_2xl()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(rgb(0xB0B0B0))
                                .child("No Tabs Open")
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0x888888))
                                .child("You don't have any tabs open. Create a new one to get started!")
                        )
                )
                .child(
                    div()
                        .mt_8()
                        .flex()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_5()
                                .py_2()
                                .bg(rgb(0x2A2A2A))
                                .shadow_md()
                                .cursor_pointer()
                                .hover(|style| style.bg(rgb(0x333344)))
                                .on_mouse_down(MouseButton::Left, move |_, cx| {
                                    let current_state = SHOW_NEW_TAB_DROPDOWN.load(Ordering::Relaxed);
                                    SHOW_NEW_TAB_DROPDOWN.store(1 - current_state, Ordering::Relaxed);
                                    cx.refresh();
                                })
                                .child(div().text_lg().child("➕"))
                                .child(div().text_sm().child("Create New Tab").text_color(rgb(0xE0E0E0)))
                        )
                        .when(TabSystem::has_closed_tabs(), |el| {
                            el.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_5()
                                    .py_2()
                                    .bg(rgb(0x23232A))
                                    .rounded_full()
                                    .shadow_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(rgb(0x2A2A33)))
                                    .on_mouse_down(MouseButton::Left, move |_, cx| {
                                        TabSystem::reopen_last_closed_tab();
                                        cx.refresh();
                                    })
                                    .child(div().text_lg().child("↶"))
                                    .child(div().text_sm().child("Reopen Closed Tab"))
                            )
                        })
                )
        }
    }
}

impl FocusableView for TabSystem {
    fn focus_handle(&self, _cx: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TabSystem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .key_context("TabSystem")
            .track_focus(&self.focus_handle)
            // Add a mouse down handler to the root to close menus when clicking away.
            .on_mouse_down(MouseButton::Left, |_, cx| {
                if SHOW_CONTEXT_MENU.load(Ordering::Relaxed) == 1 {
                    SHOW_CONTEXT_MENU.store(0, Ordering::Relaxed);
                    cx.refresh();
                }
                if SHOW_NEW_TAB_DROPDOWN.load(Ordering::Relaxed) == 1 {
                    SHOW_NEW_TAB_DROPDOWN.store(0, Ordering::Relaxed);
                    cx.refresh();
                }
            })
            .on_action(cx.listener(|_, _: &NextTab, cx| { Self::next_tab(); cx.refresh(); }))
            .on_action(cx.listener(|_, _: &PrevTab, cx| { Self::prev_tab(); cx.refresh(); }))
            .on_action(cx.listener(|_, _: &CloseTab, cx| {
                let selected_tab_id = SELECTED_TAB.load(Ordering::Relaxed);
                Self::close_tab(selected_tab_id);
                cx.refresh();
            }))
            .on_action(cx.listener(|_, _: &NewTab, cx| {
                // The keyboard shortcut now also toggles the dropdown.
                let current_state = SHOW_NEW_TAB_DROPDOWN.load(Ordering::Relaxed);
                SHOW_NEW_TAB_DROPDOWN.store(1 - current_state, Ordering::Relaxed);
                cx.refresh();
            }))
            .on_action(cx.listener(|_, _: &ReopenTab, cx| { Self::reopen_last_closed_tab(); cx.refresh(); }))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .child(self.render_tab_bar(cx))
                    .child(self.render_content(cx))
            )
            // Render the menus on top of the main content.
            .child(self.render_context_menu(cx))
            .child(self.render_new_tab_dropdown(cx))
    }
}

// --- Example Content Providers and their Registrable Types ---
// These are left here as examples. You can move them to other files.
// To use them, you must call `register_tab_type` for each one.

// --- Text Tab ---
pub struct TextTabType;
impl RegisterableTab for TextTabType {
    fn name(&self) -> &'static str { "Text Document" }
    fn create(&self) -> Arc<dyn TabContentProvider> {
        Arc::new(TextContentProvider::new("New Document".to_string(), "".to_string()))
    }
    fn icon(&self) -> Option<String> { Some("📝".to_string()) }
}

// --- Level Editor Tab ---
#[derive(Clone)]
pub struct LevelEditorContentProvider { title: String, level_data: String, is_dirty: bool }
impl LevelEditorContentProvider { pub fn new(title: String) -> Self { Self { title, level_data: "Level data...".to_string(), is_dirty: false } } }
impl TabContentProvider for LevelEditorContentProvider {
    fn render_content(&self, _tab_id: usize) -> AnyElement { div().p_4().child(format!("Level Editor: {}", self.title)).into_any_element() }
    fn get_title(&self) -> String { self.title.clone() }
    fn is_dirty(&self) -> bool { self.is_dirty }
    fn can_close(&self) -> bool { true }
}
pub struct LevelEditorTabType;
impl RegisterableTab for LevelEditorTabType {
    fn name(&self) -> &'static str { "Level Editor" }
    fn create(&self) -> Arc<dyn TabContentProvider> { Arc::new(LevelEditorContentProvider::new("New Level".to_string())) }
    fn icon(&self) -> Option<String> { Some("🎮".to_string()) }
}

// --- Settings Tab ---
#[derive(Clone)]
pub struct SettingsContentProvider { title: String, is_dirty: bool }
impl SettingsContentProvider { pub fn new() -> Self { Self { title: "Settings".to_string(), is_dirty: false } } }
impl TabContentProvider for SettingsContentProvider {
    fn render_content(&self, _tab_id: usize) -> AnyElement { div().p_4().child("Application Settings").into_any_element() }
    fn get_title(&self) -> String { self.title.clone() }
    fn is_dirty(&self) -> bool { self.is_dirty }
    fn can_close(&self) -> bool { false } // Settings tab cannot be closed
}
pub struct SettingsTabType;
impl RegisterableTab for SettingsTabType {
    fn name(&self) -> &'static str { "Settings" }
    fn create(&self) -> Arc<dyn TabContentProvider> { Arc::new(SettingsContentProvider::new()) }
    fn icon(&self) -> Option<String> { Some("⚙️".to_string()) }
}


// --- System Setup ---

pub fn register_tab_shortcuts(cx: &mut AppContext) {
    cx.bind_keys([
        gpui::KeyBinding::new("cmd-t", NewTab, None),
        gpui::KeyBinding::new("ctrl-t", NewTab, None),
        gpui::KeyBinding::new("cmd-w", CloseTab, None),
        gpui::KeyBinding::new("ctrl-w", CloseTab, None),
        gpui::KeyBinding::new("cmd-shift-t", ReopenTab, None),
        gpui::KeyBinding::new("ctrl-shift-t", ReopenTab, None),
        gpui::KeyBinding::new("ctrl-tab", NextTab, None),
        gpui::KeyBinding::new("ctrl-shift-tab", PrevTab, None),
    ]);
}

/// Creates a new, empty TabSystem.
pub fn create_tab_system(cx: &mut ViewContext<impl Render>) -> View<TabSystem> {
    cx.new_view(|cx| TabSystem::new(cx))
}

// --- Utility Functions ---
// These are helpful for interacting with the tab system from outside.
impl TabSystem {
    pub fn get_selected_tab_id() -> Option<usize> {
        let id = SELECTED_TAB.load(Ordering::Relaxed);
        if OPEN_TABS.lock().unwrap().iter().any(|t| t.id == id) {
            Some(id)
        } else {
            None
        }
    }
    
    pub fn get_tab_count() -> usize {
        OPEN_TABS.lock().unwrap().len()
    }
    
    pub fn has_closed_tabs() -> bool {
        !CLOSED_TABS.lock().unwrap().is_empty()
    }
}
