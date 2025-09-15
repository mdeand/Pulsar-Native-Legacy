use imgui::*;
use crate::ui::theme::{PulsarTheme, ButtonVariant, VisualEffects};
use std::collections::HashMap;

/// Represents a draggable, dockable panel that can be detached into floating windows
#[derive(Debug, Clone)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub content_type: PanelContentType,
    pub is_open: bool,
    pub is_floating: bool,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub min_size: [f32; 2],
    pub dock_id: Option<u32>,
    pub can_close: bool,
    pub is_modified: bool,
}

#[derive(Debug, Clone)]
pub enum PanelContentType {
    SceneHierarchy,
    Properties,
    AssetBrowser,
    Console,
    Profiler,
    LevelEditor,
    ScriptEditor,
    BlueprintEditor,
    MaterialEditor,
    AnimationEditor,
    ParticleEditor,
    SoundEditor,
    TerrainEditor,
    NavMeshEditor,
    PhysicsDebug,
}

/// Manages the panel system with docking, floating, and layout persistence
pub struct PanelManager {
    panels: HashMap<String, Panel>,
    floating_windows: Vec<String>,
    dock_layout: DockLayout,
    active_panel: Option<String>,
    panel_order: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DockLayout {
    pub left_panels: Vec<String>,
    pub right_panels: Vec<String>,
    pub bottom_panels: Vec<String>,
    pub center_tabs: Vec<String>,
    pub left_width: f32,
    pub right_width: f32,
    pub bottom_height: f32,
}

impl Default for DockLayout {
    fn default() -> Self {
        Self {
            left_panels: vec!["scene_hierarchy".to_string()],
            right_panels: vec!["properties".to_string(), "asset_browser".to_string()],
            bottom_panels: vec!["console".to_string()],
            center_tabs: vec!["level_editor".to_string(), "script_editor".to_string()],
            left_width: 280.0,
            right_width: 320.0,
            bottom_height: 200.0,
        }
    }
}

impl PanelManager {
    pub fn new() -> Self {
        let mut panels = HashMap::new();

        // Initialize default panels
        let default_panels = vec![
            Panel {
                id: "scene_hierarchy".to_string(),
                title: "Scene Hierarchy".to_string(),
                icon: "🌳".to_string(),
                content_type: PanelContentType::SceneHierarchy,
                is_open: true,
                is_floating: false,
                position: [0.0, 0.0],
                size: [280.0, 400.0],
                min_size: [200.0, 200.0],
                dock_id: None,
                can_close: true,
                is_modified: false,
            },
            Panel {
                id: "properties".to_string(),
                title: "Properties".to_string(),
                icon: "🔧".to_string(),
                content_type: PanelContentType::Properties,
                is_open: true,
                is_floating: false,
                position: [0.0, 0.0],
                size: [320.0, 500.0],
                min_size: [250.0, 200.0],
                dock_id: None,
                can_close: true,
                is_modified: false,
            },
            Panel {
                id: "asset_browser".to_string(),
                title: "Asset Browser".to_string(),
                icon: "📁".to_string(),
                content_type: PanelContentType::AssetBrowser,
                is_open: true,
                is_floating: false,
                position: [0.0, 0.0],
                size: [320.0, 300.0],
                min_size: [300.0, 150.0],
                dock_id: None,
                can_close: true,
                is_modified: false,
            },
            Panel {
                id: "console".to_string(),
                title: "Console".to_string(),
                icon: "📋".to_string(),
                content_type: PanelContentType::Console,
                is_open: true,
                is_floating: false,
                position: [0.0, 0.0],
                size: [800.0, 200.0],
                min_size: [400.0, 100.0],
                dock_id: None,
                can_close: true,
                is_modified: false,
            },
            Panel {
                id: "level_editor".to_string(),
                title: "Level Editor".to_string(),
                icon: "🗺️".to_string(),
                content_type: PanelContentType::LevelEditor,
                is_open: true,
                is_floating: false,
                position: [0.0, 0.0],
                size: [800.0, 600.0],
                min_size: [400.0, 300.0],
                dock_id: None,
                can_close: false,
                is_modified: false,
            },
            Panel {
                id: "script_editor".to_string(),
                title: "Script Editor".to_string(),
                icon: "📜".to_string(),
                content_type: PanelContentType::ScriptEditor,
                is_open: true,
                is_floating: false,
                position: [0.0, 0.0],
                size: [800.0, 600.0],
                min_size: [400.0, 300.0],
                dock_id: None,
                can_close: true,
                is_modified: true,
            },
        ];

        for panel in default_panels {
            panels.insert(panel.id.clone(), panel);
        }

        let panel_order = vec![
            "scene_hierarchy".to_string(),
            "properties".to_string(),
            "asset_browser".to_string(),
            "console".to_string(),
            "level_editor".to_string(),
            "script_editor".to_string(),
        ];

        Self {
            panels,
            floating_windows: Vec::new(),
            dock_layout: DockLayout::default(),
            active_panel: Some("level_editor".to_string()),
            panel_order,
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        let display_size = ui.io().display_size;
        let menu_height = ui.frame_height();

        // Create main dockspace
        self.render_dockspace(ui, display_size, menu_height);

        // Render floating windows
        self.render_floating_windows(ui);
    }

    fn render_dockspace(&mut self, ui: &Ui, display_size: [f32; 2], menu_height: f32) {
        let dockspace_pos = [0.0, menu_height];
        let dockspace_size = [display_size[0], display_size[1] - menu_height];

        ui.window("DockSpace")
            .position(dockspace_pos, Condition::Always)
            .size(dockspace_size, Condition::Always)
            .flags(
                WindowFlags::NO_DECORATION
                | WindowFlags::NO_MOVE
                | WindowFlags::NO_RESIZE
                | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
                | WindowFlags::NO_NAV_FOCUS
                | WindowFlags::NO_BACKGROUND
            )
            .build(|| {
                // Use columns for docking layout since we're using imgui 0.10.0
                self.render_docked_layout(ui, dockspace_size);
            });
    }

    fn render_docked_layout(&mut self, ui: &Ui, available_size: [f32; 2]) {
        // Three column layout: Left | Center | Right
        ui.columns(3, "MainDockLayout", true);

        // Left column
        ui.set_column_width(0, self.dock_layout.left_width);
        self.render_left_panels(ui);

        ui.next_column();

        // Center column with tabs and bottom panel
        let center_width = available_size[0] - self.dock_layout.left_width - self.dock_layout.right_width;
        ui.set_column_width(1, center_width);
        self.render_center_area(ui, available_size[1]);

        ui.next_column();

        // Right column
        ui.set_column_width(2, self.dock_layout.right_width);
        self.render_right_panels(ui);

        ui.columns(1, "", false);
    }

    fn render_left_panels(&mut self, ui: &Ui) {
        for panel_id in self.dock_layout.left_panels.clone() {
            if let Some(panel) = self.panels.get_mut(&panel_id) {
                if panel.is_open && !panel.is_floating {
                    self.render_panel_content(ui, panel);
                }
            }
        }
    }

    fn render_right_panels(&mut self, ui: &Ui) {
        for panel_id in self.dock_layout.right_panels.clone() {
            if let Some(panel) = self.panels.get_mut(&panel_id) {
                if panel.is_open && !panel.is_floating {
                    self.render_panel_content(ui, panel);
                }
            }
        }
    }

    fn render_center_area(&mut self, ui: &Ui, available_height: f32) {
        // Top area with tabs
        let tab_area_height = available_height - self.dock_layout.bottom_height - 40.0;

        ui.child_window("CenterTabs")
            .size([0.0, tab_area_height])
            .build(|| {
                self.render_center_tabs(ui);
            });

        // Bottom panel area
        ui.child_window("BottomPanels")
            .size([0.0, self.dock_layout.bottom_height])
            .build(|| {
                for panel_id in self.dock_layout.bottom_panels.clone() {
                    if let Some(panel) = self.panels.get_mut(&panel_id) {
                        if panel.is_open && !panel.is_floating {
                            self.render_panel_content(ui, panel);
                        }
                    }
                }
            });
    }

    fn render_center_tabs(&mut self, ui: &Ui) {
        // Custom tab bar implementation for better styling
        let tab_height = 32.0;

        ui.child_window("TabBar")
            .size([0.0, tab_height])
            .flags(WindowFlags::NO_SCROLLBAR)
            .build(|| {
                self.render_custom_tab_bar(ui);
            });

        // Active tab content
        ui.child_window("TabContent")
            .size([0.0, -1.0])
            .build(|| {
                if let Some(active_id) = &self.active_panel.clone() {
                    if let Some(panel) = self.panels.get_mut(active_id) {
                        self.render_panel_content(ui, panel);
                    }
                }
            });
    }

    fn render_custom_tab_bar(&mut self, ui: &Ui) {
        let mut tab_to_close: Option<String> = None;

        for (index, panel_id) in self.dock_layout.center_tabs.iter().enumerate() {
            if let Some(panel) = self.panels.get(panel_id) {
                if !panel.is_open || panel.is_floating {
                    continue;
                }

                if index > 0 {
                    ui.same_line();
                }

                let is_active = self.active_panel.as_ref() == Some(panel_id);

                // Tab styling
                let style_token = if is_active {
                    Some(ui.push_style_color(StyleColor::Button, PulsarTheme::TAB_ACTIVE))
                } else {
                    Some(ui.push_style_color(StyleColor::Button, PulsarTheme::TAB_INACTIVE))
                };

                let hover_token = if is_active {
                    Some(ui.push_style_color(StyleColor::ButtonHovered, PulsarTheme::BLUE_HOVER))
                } else {
                    Some(ui.push_style_color(StyleColor::ButtonHovered, PulsarTheme::TAB_HOVER))
                };

                // Tab label with icon and modified indicator
                let modified_indicator = if panel.is_modified { "●" } else { "" };
                let tab_label = format!("{} {} {}", panel.icon, panel.title, modified_indicator);

                if ui.button(&tab_label) {
                    self.active_panel = Some(panel_id.clone());
                }

                if let Some(token) = hover_token {
                    token.pop();
                }
                if let Some(token) = style_token {
                    token.pop();
                }

                // Close button
                if panel.can_close {
                    ui.same_line();
                    let close_id = format!("×##{}", panel_id);
                    if ui.small_button(&close_id) {
                        tab_to_close = Some(panel_id.clone());
                    }
                }

                // Context menu for tab
                if ui.is_item_clicked_with_button(MouseButton::Right) {
                    ui.open_popup(&format!("tab_context_{}", panel_id));
                }

                ui.popup(&format!("tab_context_{}", panel_id), || {
                    if ui.menu_item("Float Window") {
                        self.float_panel(panel_id);
                    }
                    if panel.can_close && ui.menu_item("Close Tab") {
                        tab_to_close = Some(panel_id.clone());
                    }
                    if ui.menu_item("Close Other Tabs") {
                        // Implement close others
                    }
                });
            }
        }

        // Handle tab closing
        if let Some(panel_id) = tab_to_close {
            self.close_panel(&panel_id);
        }

        // Add new tab button
        ui.same_line();
        let add_style = ui.push_style_color(StyleColor::Button, PulsarTheme::BLUE_PRIMARY);
        if ui.small_button("+") {
            // Show panel selector
            ui.open_popup("add_panel");
        }
        add_style.pop();

        // Add panel popup
        ui.popup("add_panel", || {
            if ui.menu_item("Blueprint Editor") {
                self.add_center_tab("blueprint_editor");
            }
            if ui.menu_item("Material Editor") {
                self.add_center_tab("material_editor");
            }
            if ui.menu_item("Animation Editor") {
                self.add_center_tab("animation_editor");
            }
        });
    }

    fn render_floating_windows(&mut self, ui: &Ui) {
        let floating_ids: Vec<String> = self.floating_windows.clone();

        for panel_id in floating_ids {
            if let Some(panel) = self.panels.get_mut(&panel_id) {
                if panel.is_open && panel.is_floating {
                    self.render_floating_panel(ui, panel);
                }
            }
        }
    }

    fn render_floating_panel(&mut self, ui: &Ui, panel: &mut Panel) {
        let window_flags = WindowFlags::NO_COLLAPSE | WindowFlags::RESIZE_FROM_ANY_SIDE;

        ui.window(&format!("{} {}", panel.icon, panel.title))
            .position(panel.position, Condition::FirstUseEver)
            .size(panel.size, Condition::FirstUseEver)
            .size_constraints(panel.min_size, [f32::INFINITY, f32::INFINITY])
            .flags(window_flags)
            .build(|| {
                // Update panel position and size
                panel.position = ui.window_pos();
                panel.size = ui.window_size();

                self.render_panel_content(ui, panel);
            });
    }

    fn render_panel_content(&self, ui: &Ui, panel: &Panel) {
        // Render the actual panel content based on type
        match panel.content_type {
            PanelContentType::SceneHierarchy => self.render_scene_hierarchy(ui),
            PanelContentType::Properties => self.render_properties(ui),
            PanelContentType::AssetBrowser => self.render_asset_browser(ui),
            PanelContentType::Console => self.render_console(ui),
            PanelContentType::Profiler => self.render_profiler(ui),
            PanelContentType::LevelEditor => self.render_level_editor(ui),
            PanelContentType::ScriptEditor => self.render_script_editor(ui),
            PanelContentType::BlueprintEditor => self.render_blueprint_editor(ui),
            PanelContentType::MaterialEditor => self.render_material_editor(ui),
            PanelContentType::AnimationEditor => self.render_animation_editor(ui),
            PanelContentType::ParticleEditor => self.render_particle_editor(ui),
            PanelContentType::SoundEditor => self.render_sound_editor(ui),
            PanelContentType::TerrainEditor => self.render_terrain_editor(ui),
            PanelContentType::NavMeshEditor => self.render_navmesh_editor(ui),
            PanelContentType::PhysicsDebug => self.render_physics_debug(ui),
        }
    }

    // Panel management methods
    pub fn float_panel(&mut self, panel_id: &str) {
        if let Some(panel) = self.panels.get_mut(panel_id) {
            panel.is_floating = true;
            self.floating_windows.push(panel_id.to_string());

            // Remove from dock layout
            self.dock_layout.left_panels.retain(|id| id != panel_id);
            self.dock_layout.right_panels.retain(|id| id != panel_id);
            self.dock_layout.bottom_panels.retain(|id| id != panel_id);
            self.dock_layout.center_tabs.retain(|id| id != panel_id);
        }
    }

    pub fn dock_panel(&mut self, panel_id: &str, dock_area: DockArea) {
        if let Some(panel) = self.panels.get_mut(panel_id) {
            panel.is_floating = false;
            self.floating_windows.retain(|id| id != panel_id);

            // Add to appropriate dock area
            match dock_area {
                DockArea::Left => self.dock_layout.left_panels.push(panel_id.to_string()),
                DockArea::Right => self.dock_layout.right_panels.push(panel_id.to_string()),
                DockArea::Bottom => self.dock_layout.bottom_panels.push(panel_id.to_string()),
                DockArea::Center => self.dock_layout.center_tabs.push(panel_id.to_string()),
            }
        }
    }

    pub fn close_panel(&mut self, panel_id: &str) {
        if let Some(panel) = self.panels.get_mut(panel_id) {
            if panel.can_close {
                panel.is_open = false;
                self.floating_windows.retain(|id| id != panel_id);

                // Remove from dock layout
                self.dock_layout.left_panels.retain(|id| id != panel_id);
                self.dock_layout.right_panels.retain(|id| id != panel_id);
                self.dock_layout.bottom_panels.retain(|id| id != panel_id);
                self.dock_layout.center_tabs.retain(|id| id != panel_id);

                // If this was the active panel, switch to another
                if self.active_panel.as_ref() == Some(panel_id) {
                    self.active_panel = self.dock_layout.center_tabs.first().cloned();
                }
            }
        }
    }

    pub fn add_center_tab(&mut self, panel_id: &str) {
        if !self.dock_layout.center_tabs.contains(&panel_id.to_string()) {
            self.dock_layout.center_tabs.push(panel_id.to_string());
            self.active_panel = Some(panel_id.to_string());
        }
    }

    // Content rendering methods (will be moved to separate files)
    fn render_scene_hierarchy(&self, ui: &Ui) {
        ui.text("🌳 Scene Objects");
        ui.separator();
        // Scene hierarchy content...
    }

    fn render_properties(&self, ui: &Ui) {
        ui.text("🔧 Properties");
        ui.separator();
        // Properties content...
    }

    fn render_asset_browser(&self, ui: &Ui) {
        ui.text("📁 Asset Browser");
        ui.separator();
        // Asset browser content...
    }

    fn render_console(&self, ui: &Ui) {
        ui.text("📋 Console");
        ui.separator();
        // Console content...
    }

    fn render_profiler(&self, ui: &Ui) {
        ui.text("📊 Profiler");
        ui.separator();
        // Profiler content...
    }

    fn render_level_editor(&self, ui: &Ui) {
        ui.text("🗺️ Level Editor");
        ui.separator();
        // Level editor content...
    }

    fn render_script_editor(&self, ui: &Ui) {
        ui.text("📜 Script Editor");
        ui.separator();
        // Script editor content...
    }

    fn render_blueprint_editor(&self, ui: &Ui) {
        ui.text("🔧 Blueprint Editor");
        ui.separator();
        // Blueprint editor content...
    }

    fn render_material_editor(&self, ui: &Ui) {
        ui.text("🎨 Material Editor");
        ui.separator();
        // Material editor content...
    }

    fn render_animation_editor(&self, ui: &Ui) {
        ui.text("🎭 Animation Editor");
        ui.separator();
        // Animation editor content...
    }

    fn render_particle_editor(&self, ui: &Ui) {
        ui.text("✨ Particle Editor");
        ui.separator();
        // Particle editor content...
    }

    fn render_sound_editor(&self, ui: &Ui) {
        ui.text("🎵 Sound Editor");
        ui.separator();
        // Sound editor content...
    }

    fn render_terrain_editor(&self, ui: &Ui) {
        ui.text("🏔️ Terrain Editor");
        ui.separator();
        // Terrain editor content...
    }

    fn render_navmesh_editor(&self, ui: &Ui) {
        ui.text("🧭 NavMesh Editor");
        ui.separator();
        // NavMesh editor content...
    }

    fn render_physics_debug(&self, ui: &Ui) {
        ui.text("⚡ Physics Debug");
        ui.separator();
        // Physics debug content...
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DockArea {
    Left,
    Right,
    Bottom,
    Center,
}