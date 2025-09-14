use imgui::*;
use std::collections::HashMap;

pub trait TabContent {
    fn render(&mut self, ui: &Ui);
    fn get_title(&self) -> &str;
    fn is_dirty(&self) -> bool;
    fn can_close(&self) -> bool { true }
    fn get_icon(&self) -> Option<&str> { None }
}

pub struct TabSystem {
    tabs: Vec<Box<dyn TabContent>>,
    tab_types: HashMap<String, (String, Box<dyn Fn() -> Box<dyn TabContent>>)>, // name -> (icon, factory)
    selected_tab: usize,
    next_tab_id: usize,
    pub show_new_tab_popup: bool,
    tab_to_close: Option<usize>,
    context_menu_tab: Option<usize>,
    context_menu_pos: [f32; 2],
}

impl TabSystem {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            tab_types: HashMap::new(),
            selected_tab: 0,
            next_tab_id: 0,
            show_new_tab_popup: false,
            tab_to_close: None,
            context_menu_tab: None,
            context_menu_pos: [0.0, 0.0],
        }
    }

    pub fn register_tab_type<F>(&mut self, name: &str, icon: &str, factory: F)
    where
        F: Fn() -> Box<dyn TabContent> + 'static
    {
        self.tab_types.insert(name.to_string(), (icon.to_string(), Box::new(factory)));
    }

    pub fn add_tab<T>(&mut self, _tab_type: &str, content: T)
    where
        T: TabContent + 'static
    {
        self.tabs.push(Box::new(content));
        self.selected_tab = self.tabs.len().saturating_sub(1);
        self.next_tab_id += 1;
    }

    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() && self.tabs[index].can_close() {
            self.tabs.remove(index);
            if self.selected_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.selected_tab = self.tabs.len() - 1;
            } else if self.tabs.is_empty() {
                self.selected_tab = 0;
            } else if self.selected_tab > index {
                self.selected_tab -= 1;
            }
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        let window_size = ui.content_region_avail();

        // Tab bar
        let tab_bar_height = ui.frame_height();

        ui.child_window("TabBar")
            .size([0.0, tab_bar_height])
            .flags(WindowFlags::NO_SCROLLBAR | WindowFlags::NO_SCROLL_WITH_MOUSE)
            .build(|| {
                // Left side - tabs (using simple same_line instead of columns)
                for (i, tab) in self.tabs.iter().enumerate() {
                    let is_selected = i == self.selected_tab;

                    if i > 0 { ui.same_line(); }

                    let mut button_color = if is_selected {
                        [0.2, 0.4, 0.8, 1.0]
                    } else {
                        [0.2, 0.2, 0.2, 1.0]
                    };

                    let _style_color = ui.push_style_color(StyleColor::Button, button_color);
                    let _style_color2 = ui.push_style_color(
                        StyleColor::ButtonHovered,
                        [button_color[0] + 0.1, button_color[1] + 0.1, button_color[2] + 0.1, 1.0]
                    );

                    let tab_text = if tab.is_dirty() {
                        format!("{}* {}", tab.get_icon().unwrap_or(""), tab.get_title())
                    } else {
                        format!("{} {}", tab.get_icon().unwrap_or(""), tab.get_title())
                    };

                    if ui.button(&tab_text) {
                        self.selected_tab = i;
                    }

                    // Right-click context menu
                    if ui.is_item_clicked_with_button(MouseButton::Right) {
                        self.context_menu_tab = Some(i);
                        self.context_menu_pos = ui.io().mouse_pos;
                        ui.open_popup("tab_context_menu");
                    }

                    // Close button
                    if tab.can_close() {
                        ui.same_line();
                        let _id = ui.push_id(&format!("close_{}", i));
                        if ui.small_button("×") {
                            self.tab_to_close = Some(i);
                        }
                    }
                }

                // Right side - new tab button (push to far right)
                ui.same_line();
                let window_width = ui.content_region_avail()[0];
                let button_width = ui.calc_text_size("+")[0] + 20.0;
                if window_width > button_width {
                    ui.set_cursor_pos([ui.cursor_pos()[0] + window_width - button_width, ui.cursor_pos()[1]]);
                }
                if ui.button("+") {
                    self.show_new_tab_popup = true;
                }
            });

        // Handle tab closing
        if let Some(index) = self.tab_to_close.take() {
            self.close_tab(index);
        }

        // Context menu
        ui.popup("tab_context_menu", || {
            if let Some(tab_index) = self.context_menu_tab {
                if ui.menu_item("Close Tab") && tab_index < self.tabs.len() {
                    self.tab_to_close = Some(tab_index);
                    self.context_menu_tab = None;
                }

                if ui.menu_item("Close Other Tabs") {
                    let mut indices_to_close = Vec::new();
                    for (i, tab) in self.tabs.iter().enumerate() {
                        if i != tab_index && tab.can_close() {
                            indices_to_close.push(i);
                        }
                    }
                    // Close from highest index to lowest to maintain indices
                    indices_to_close.sort_by(|a, b| b.cmp(a));
                    for idx in indices_to_close {
                        self.close_tab(idx);
                    }
                    self.context_menu_tab = None;
                }
            }
        });

        // New tab popup
        if self.show_new_tab_popup {
            ui.open_popup("new_tab_popup");
            self.show_new_tab_popup = false;
        }

        ui.popup("new_tab_popup", || {
                ui.text("Select Tab Type");
                ui.separator();

                for (name, (icon, factory)) in &self.tab_types {
                    if ui.button(&format!("{} {}", icon, name)) {
                        let new_tab = factory();
                        self.tabs.push(new_tab);
                        self.selected_tab = self.tabs.len() - 1;
                        ui.close_current_popup();
                    }
                }

                ui.separator();
                if ui.button("Cancel") {
                    ui.close_current_popup();
                }
        });

        ui.separator();

        // Content area
        if !self.tabs.is_empty() && self.selected_tab < self.tabs.len() {
            let remaining_size = [window_size[0], window_size[1] - tab_bar_height - 20.0];
            ui.child_window("TabContent")
                .size(remaining_size)
                .build(|| {
                    self.tabs[self.selected_tab].render(ui);
                });
        } else {
            // No tabs open
            let text_size = ui.calc_text_size("No tabs open");
            let center_pos = [
                (window_size[0] - text_size[0]) * 0.5,
                (window_size[1] - text_size[1]) * 0.5
            ];
            ui.set_cursor_pos(center_pos);
            ui.text("No tabs open");
            ui.text("Click the '+' button to create a new tab");
        }
    }
}

// Example tab implementations
pub struct TextEditor {
    title: String,
    content: String,
    is_dirty: bool,
}

impl TextEditor {
    pub fn new(title: String) -> Self {
        Self {
            title,
            content: String::new(),
            is_dirty: false,
        }
    }
}

impl TabContent for TextEditor {
    fn render(&mut self, ui: &Ui) {
        ui.text(&format!("Text Editor: {}", self.title));
        ui.separator();

        if ui.input_text_multiline("Content", &mut self.content, ui.content_region_avail())
            .build()
        {
            self.is_dirty = true;
        }
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    fn get_icon(&self) -> Option<&str> {
        Some("📝")
    }
}

pub struct Settings {
    title: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            title: "Settings".to_string(),
        }
    }
}

impl TabContent for Settings {
    fn render(&mut self, ui: &Ui) {
        ui.text("Application Settings");
        ui.separator();

        ui.text("Graphics Settings");
        ui.checkbox("VSync", &mut true);
        ui.checkbox("Fullscreen", &mut false);

        ui.separator();
        ui.text("Audio Settings");
        ui.slider_config("Master Volume", 0.0, 1.0)
            .build(&mut 0.8);
        ui.slider_config("SFX Volume", 0.0, 1.0)
            .build(&mut 0.6);
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn is_dirty(&self) -> bool {
        false
    }

    fn can_close(&self) -> bool {
        false // Settings tab cannot be closed
    }

    fn get_icon(&self) -> Option<&str> {
        Some("⚙️")
    }
}