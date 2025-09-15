use imgui::*;
use crate::ui::theme::PulsarTheme;

/// Simple AMOLED UI that works with imgui 0.10.0
pub struct SimpleGameUI {
    show_hierarchy: bool,
    show_inspector: bool,
    show_console: bool,
    show_asset_browser: bool,
    // Panel sizes for resizing
    left_panel_width: f32,
    right_panel_width: f32,
    bottom_panel_height: f32,
    // Tab system
    active_tab: EditorTab,
    available_tabs: Vec<EditorTab>,
    show_tab_search: bool,
    tab_search_query: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditorTab {
    LevelEditor,
    ScriptEditor,
    BlueprintEditor,
    MaterialEditor,
    AnimationEditor,
    ParticleEditor,
    AudioEditor,
    TerrainEditor,
    PhysicsDebug,
    Profiler,
}

impl EditorTab {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::LevelEditor => "Level Editor",
            Self::ScriptEditor => "Script Editor",
            Self::BlueprintEditor => "Blueprint Editor",
            Self::MaterialEditor => "Material Editor",
            Self::AnimationEditor => "Animation Editor",
            Self::ParticleEditor => "Particle Editor",
            Self::AudioEditor => "Audio Editor",
            Self::TerrainEditor => "Terrain Editor",
            Self::PhysicsDebug => "Physics Debug",
            Self::Profiler => "Profiler",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::LevelEditor => "🌍",
            Self::ScriptEditor => "📜",
            Self::BlueprintEditor => "🔧",
            Self::MaterialEditor => "🎨",
            Self::AnimationEditor => "🎬",
            Self::ParticleEditor => "✨",
            Self::AudioEditor => "🔊",
            Self::TerrainEditor => "🏔️",
            Self::PhysicsDebug => "⚡",
            Self::Profiler => "📊",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::LevelEditor => "Design and build game levels",
            Self::ScriptEditor => "Write and debug game scripts",
            Self::BlueprintEditor => "Visual scripting system",
            Self::MaterialEditor => "Create and edit materials",
            Self::AnimationEditor => "Create character animations",
            Self::ParticleEditor => "Design particle effects",
            Self::AudioEditor => "Edit and manage audio",
            Self::TerrainEditor => "Sculpt and paint terrain",
            Self::PhysicsDebug => "Debug physics simulation",
            Self::Profiler => "Performance monitoring",
        }
    }

    pub fn all_tabs() -> Vec<Self> {
        vec![
            Self::LevelEditor,
            Self::ScriptEditor,
            Self::BlueprintEditor,
            Self::MaterialEditor,
            Self::AnimationEditor,
            Self::ParticleEditor,
            Self::AudioEditor,
            Self::TerrainEditor,
            Self::PhysicsDebug,
            Self::Profiler,
        ]
    }
}

impl SimpleGameUI {
    pub fn new() -> Self {
        Self {
            show_hierarchy: true,
            show_inspector: true,
            show_console: true,
            show_asset_browser: true,
            left_panel_width: 250.0,
            right_panel_width: 300.0,
            bottom_panel_height: 200.0,
            active_tab: EditorTab::LevelEditor,  // Default to Level Editor
            available_tabs: vec![EditorTab::LevelEditor],  // Start with Level Editor open
            show_tab_search: false,
            tab_search_query: String::new(),
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        // Ensure display size is valid to avoid ClipRect assertion
        let display_size = ui.io().display_size;
        if display_size[0] <= 0.0 || display_size[1] <= 0.0 {
            return;
        }

        // Main menu bar
        self.render_main_menu_bar(ui);

        // Calculate available space
        let menu_height = ui.frame_height();
        let available_height = display_size[1] - menu_height;
        let available_width = display_size[0];

        // Update responsive panel sizing
        self.update_responsive_panel_sizes(available_width, available_height);

        // Render tab bar
        self.render_tab_bar(ui, menu_height, available_width);

        // Render panels as separate windows with proper positioning
        let tab_bar_height = 35.0;
        self.render_separate_panels(ui, menu_height + tab_bar_height, available_width, available_height - tab_bar_height);

        // Render tab search modal if open (render last for proper z-order)
        if self.show_tab_search {
            self.render_tab_search_modal(ui);
        }
    }

    fn render_main_menu_bar(&mut self, ui: &Ui) {
        if let Some(_menu_bar) = ui.begin_main_menu_bar() {
            // File menu
            if let Some(_file_menu) = ui.begin_menu("File") {
                if ui.menu_item("New Project") {}
                if ui.menu_item("Open Project") {}
                if ui.menu_item("Save Project") {}
                ui.separator();
                if ui.menu_item("Exit") {
                    std::process::exit(0);
                }
            }

            // View menu
            if let Some(_view_menu) = ui.begin_menu("View") {
                if ui.menu_item_config("Hierarchy")
                    .selected(self.show_hierarchy)
                    .build()
                {
                    self.show_hierarchy = !self.show_hierarchy;
                }
                if ui.menu_item_config("Inspector")
                    .selected(self.show_inspector)
                    .build()
                {
                    self.show_inspector = !self.show_inspector;
                }
                if ui.menu_item_config("Console")
                    .selected(self.show_console)
                    .build()
                {
                    self.show_console = !self.show_console;
                }
                if ui.menu_item_config("Asset Browser")
                    .selected(self.show_asset_browser)
                    .build()
                {
                    self.show_asset_browser = !self.show_asset_browser;
                }
            }

            // Help menu
            if let Some(_help_menu) = ui.begin_menu("Help") {
                if ui.menu_item("About") {}
            }

            // FPS counter
            let fps = crate::frame_counter::get_fps();
            let fps_text = format!("FPS: {}", fps);
            let text_size = ui.calc_text_size(&fps_text);
            let menu_width = ui.io().display_size[0];
            ui.set_cursor_pos([menu_width - text_size[0] - 16.0, 4.0]);
            ui.text_colored(PulsarTheme::TEXT_SECONDARY, &fps_text);
        }
    }

    fn render_separate_panels(&mut self, ui: &Ui, menu_height: f32, available_width: f32, available_height: f32) {
        // Calculate panel positions and sizes
        let center_x = self.left_panel_width;
        let center_width = available_width - self.left_panel_width - self.right_panel_width;
        let center_height = available_height - if self.show_console || self.show_asset_browser { self.bottom_panel_height } else { 0.0 };

        // Left panel - Hierarchy
        if self.show_hierarchy {
            ui.window("Hierarchy")
                .position([0.0, menu_height], Condition::FirstUseEver)
                .size([self.left_panel_width, center_height], Condition::FirstUseEver)
                .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
                .resizable(true)
                .build(|| {
                    self.render_hierarchy_content(ui);
                });
        }

        // Center panel - Active tab content (always visible)
        let window_title = format!("{} - {}", self.active_tab.icon(), self.active_tab.display_name());
        ui.window(&window_title)
            .position([center_x, menu_height], Condition::Always)
            .size([center_width.max(200.0), center_height], Condition::Always)
            .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE | WindowFlags::NO_RESIZE)
            .build(|| {
                self.render_active_tab_content(ui);
            });

        // Right panel - Inspector
        if self.show_inspector {
            ui.window("Inspector")
                .position([center_x + center_width, menu_height], Condition::FirstUseEver)
                .size([self.right_panel_width, center_height], Condition::FirstUseEver)
                .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
                .resizable(true)
                .build(|| {
                    self.render_inspector_content(ui);
                });
        }

        // Bottom panels (render these last to ensure proper Z-order)
        let bottom_y = menu_height + center_height;
        let bottom_width = available_width;

        if self.show_console && self.show_asset_browser {
            // Split bottom area between console and asset browser
            let console_width = bottom_width * 0.6;
            let asset_width = bottom_width * 0.4;

            ui.window("Console")
                .position([0.0, bottom_y], Condition::FirstUseEver)
                .size([console_width, self.bottom_panel_height], Condition::FirstUseEver)
                .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
                .resizable(true)
                .build(|| {
                    self.render_console_content(ui);
                });

            ui.window("Asset Browser")
                .position([console_width, bottom_y], Condition::FirstUseEver)
                .size([asset_width, self.bottom_panel_height], Condition::FirstUseEver)
                .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
                .resizable(true)
                .build(|| {
                    self.render_asset_browser_content(ui);
                });
        } else if self.show_console {
            ui.window("Console")
                .position([0.0, bottom_y], Condition::FirstUseEver)
                .size([bottom_width, self.bottom_panel_height], Condition::FirstUseEver)
                .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
                .resizable(true)
                .build(|| {
                    self.render_console_content(ui);
                });
        } else if self.show_asset_browser {
            ui.window("Asset Browser")
                .position([0.0, bottom_y], Condition::FirstUseEver)
                .size([bottom_width, self.bottom_panel_height], Condition::FirstUseEver)
                .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
                .resizable(true)
                .build(|| {
                    self.render_asset_browser_content(ui);
                });
        }
    }

    fn render_hierarchy_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🌳 Scene Hierarchy");
        ui.separator();

        if let Some(_tree) = ui.tree_node("📁 Scene Root") {
            if let Some(_player_tree) = ui.tree_node("🎮 Player") {
                ui.text("🎯 PlayerController");
                ui.text("📷 Camera");
                ui.text("🎭 MeshRenderer");
            }
            if let Some(_env_tree) = ui.tree_node("🌍 Environment") {
                ui.text("☀️ DirectionalLight");
                ui.text("🏔️ Terrain");
                ui.text("🌊 Water");
            }
        }
    }

    fn render_inspector_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🔍 Inspector");
        ui.separator();

        ui.text("Selected: None");
        ui.spacing();

        if ui.collapsing_header("Transform", TreeNodeFlags::DEFAULT_OPEN) {
            ui.text("Position:");
            let mut pos = [0.0f32, 0.0, 0.0];
            ui.input_float3("##pos", &mut pos).build();

            ui.text("Rotation:");
            let mut rot = [0.0f32, 0.0, 0.0];
            ui.input_float3("##rot", &mut rot).build();

            ui.text("Scale:");
            let mut scale = [1.0f32, 1.0, 1.0];
            ui.input_float3("##scale", &mut scale).build();
        }
    }

    fn render_level_editor_content(&self, ui: &Ui) {
        // Main toolbar with level operations
        {
            let _toolbar_bg = ui.push_style_color(StyleColor::ChildBg, PulsarTheme::DARKER_PANEL);
            ui.child_window("LevelToolbar")
                .size([0.0, 40.0])
                .border(false)
                .build(|| {
                    ui.text("Level:");
                    ui.same_line();

                    {
                        let _btn_color = ui.push_style_color(StyleColor::Button, PulsarTheme::BLUE_PRIMARY);
                        if ui.button_with_size("🆕 New", [60.0, 28.0]) {}
                    }
                    ui.same_line();
                    if ui.button_with_size("📁 Load", [60.0, 28.0]) {}
                    ui.same_line();
                    if ui.button_with_size("💾 Save", [60.0, 28.0]) {}
                    ui.same_line();
                    ui.text("|");
                    ui.same_line();

                    {
                        let _play_color = ui.push_style_color(StyleColor::Button, [0.2, 0.7, 0.2, 1.0]);
                        if ui.button_with_size("▶ Play", [60.0, 28.0]) {}
                    }
                    ui.same_line();
                    if ui.button_with_size("🔨 Build", [60.0, 28.0]) {}
                });
        }

        ui.spacing();

        // Tool selection bar
        {
            let _tool_bg = ui.push_style_color(StyleColor::ChildBg, PulsarTheme::DARKER_PANEL);
            ui.child_window("ToolBar")
                .size([0.0, 40.0])
                .border(false)
                .build(|| {
                    ui.text("Tools:");
                    ui.same_line();

                    {
                        let _select_color = ui.push_style_color(StyleColor::Button, PulsarTheme::TAB_ACTIVE);
                        if ui.button_with_size("🎩 Select", [70.0, 28.0]) {}
                    }
                    ui.same_line();
                    if ui.button_with_size("↔ Move", [60.0, 28.0]) {}
                    ui.same_line();
                    if ui.button_with_size("🔄 Rotate", [70.0, 28.0]) {}
                    ui.same_line();
                    if ui.button_with_size("🔍 Scale", [60.0, 28.0]) {}
                    ui.same_line();
                    ui.text("|");
                    ui.same_line();
                    if ui.button_with_size("🎨 Paint", [60.0, 28.0]) {}
                });
        }

        ui.spacing();

        // 3D Viewport with enhanced visuals
        let pos = ui.cursor_screen_pos();
        let avail = ui.content_region_avail();
        let size = [avail[0] - 10.0, avail[1] - 10.0];

        // Ensure positive size to avoid ClipRect assertion
        if size[0] > 0.0 && size[1] > 0.0 {
            // Main viewport background (deep space black)
            ui.get_window_draw_list()
                .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::PURE_BLACK)
                .filled(true)
                .build();

            // Grid pattern for 3D viewport (subtle blue grid)
            let grid_size = 40.0;
            let grid_color = [0.05, 0.1, 0.15, 0.8];
            for x in (0..(size[0] as i32)).step_by(grid_size as usize) {
                ui.get_window_draw_list()
                    .add_line([pos[0] + x as f32, pos[1]], [pos[0] + x as f32, pos[1] + size[1]], grid_color)
                    .build();
            }
            for y in (0..(size[1] as i32)).step_by(grid_size as usize) {
                ui.get_window_draw_list()
                    .add_line([pos[0], pos[1] + y as f32], [pos[0] + size[0], pos[1] + y as f32], grid_color)
                    .build();
            }

            // Viewport border with blue glow
            ui.get_window_draw_list()
                .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::BLUE_PRIMARY)
                .thickness(2.0)
                .build();

            // Viewport info overlay
            let info_pos = [pos[0] + 10.0, pos[1] + 10.0];
            ui.get_window_draw_list()
                .add_text(info_pos, PulsarTheme::TEXT_SECONDARY, "3D Viewport");
            ui.get_window_draw_list()
                .add_text([info_pos[0], info_pos[1] + 20.0], PulsarTheme::TEXT_MUTED, "WASD: Move | Mouse: Look | F: Focus");

            // 3D Gizmo at center (RGB axes)
            let center = [pos[0] + size[0] / 2.0, pos[1] + size[1] / 2.0];
            let gizmo_size = 15.0;

            // X-axis (Red)
            ui.get_window_draw_list()
                .add_line([center[0], center[1]], [center[0] + gizmo_size, center[1]], [1.0, 0.3, 0.3, 1.0])
                .thickness(3.0)
                .build();
            ui.get_window_draw_list()
                .add_circle([center[0] + gizmo_size, center[1]], 4.0, [1.0, 0.2, 0.2, 1.0])
                .filled(true)
                .build();

            // Y-axis (Green)
            ui.get_window_draw_list()
                .add_line([center[0], center[1]], [center[0], center[1] - gizmo_size], [0.3, 1.0, 0.3, 1.0])
                .thickness(3.0)
                .build();
            ui.get_window_draw_list()
                .add_circle([center[0], center[1] - gizmo_size], 4.0, [0.2, 1.0, 0.2, 1.0])
                .filled(true)
                .build();

            // Z-axis (Blue)
            ui.get_window_draw_list()
                .add_line([center[0], center[1]], [center[0] - gizmo_size * 0.7, center[1] + gizmo_size * 0.7], [0.3, 0.3, 1.0, 1.0])
                .thickness(3.0)
                .build();
            ui.get_window_draw_list()
                .add_circle([center[0] - gizmo_size * 0.7, center[1] + gizmo_size * 0.7], 4.0, [0.2, 0.2, 1.0, 1.0])
                .filled(true)
                .build();

            // Center origin point
            ui.get_window_draw_list()
                .add_circle(center, 6.0, PulsarTheme::TEXT_PRIMARY)
                .filled(true)
                .build();

            ui.dummy(size);
        }
    }

    fn render_console_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "💻 Console");
        ui.separator();

        ui.child_window("ConsoleOutput")
            .size([0.0, -30.0])
            .build(|| {
                ui.text_colored([0.5, 1.0, 0.5, 1.0], "[INFO] Pulsar Engine initialized");
                ui.text_colored([1.0, 1.0, 0.5, 1.0], "[WARN] Missing texture: default.png");
                ui.text_colored([1.0, 0.5, 0.5, 1.0], "[ERROR] Failed to load script: test.rs");
                ui.text_colored([0.5, 1.0, 0.5, 1.0], "[INFO] Level loaded successfully");
            });

        ui.separator();
        let mut command = String::new();
        ui.input_text("Command", &mut command).build();
    }

    fn render_asset_browser_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📁 Asset Browser");
        ui.separator();

        ui.child_window("AssetGrid")
            .size([0.0, -30.0])
            .build(|| {
                let assets = vec!["texture1.png", "model.fbx", "audio.wav", "script.rs", "material.mat"];
                for (i, asset) in assets.iter().enumerate() {
                    if i > 0 && i % 3 != 0 { ui.same_line(); }

                    let pos = ui.cursor_screen_pos();
                    let thumbnail_size = [48.0, 48.0];

                    // Asset thumbnail
                    ui.get_window_draw_list()
                        .add_rect(pos, [pos[0] + thumbnail_size[0], pos[1] + thumbnail_size[1]], PulsarTheme::DARK_PANEL)
                        .filled(true)
                        .build();

                    ui.dummy(thumbnail_size);
                    ui.text(asset);
                    if i % 3 == 2 { ui.spacing(); }
                }
            });
    }

    fn render_tab_bar(&mut self, ui: &Ui, menu_height: f32, available_width: f32) {
        ui.window("Tab Bar")
            .position([0.0, menu_height], Condition::Always)
            .size([available_width, 35.0], Condition::Always)
            .flags(WindowFlags::NO_DECORATION | WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE)
            .build(|| {
                // Render existing tabs - each in their own scope to ensure proper token cleanup
                for (i, tab) in self.available_tabs.clone().iter().enumerate() {
                    if i > 0 {
                        ui.same_line();
                    }

                    let is_active = *tab == self.active_tab;
                    let tab_label = format!("{} {}", tab.icon(), tab.display_name());

                    // Each tab button in its own scope for proper token management
                    {
                        let _button_token = if is_active {
                            ui.push_style_color(StyleColor::Button, PulsarTheme::TAB_ACTIVE)
                        } else {
                            ui.push_style_color(StyleColor::Button, PulsarTheme::TAB_INACTIVE)
                        };

                        let _hover_token = if is_active {
                            ui.push_style_color(StyleColor::ButtonHovered, PulsarTheme::BLUE_HOVER)
                        } else {
                            ui.push_style_color(StyleColor::ButtonHovered, PulsarTheme::TAB_HOVER)
                        };

                        if ui.button_with_size(&tab_label, [150.0, 28.0]) {
                            self.active_tab = tab.clone();
                        }
                        // Tokens automatically dropped at end of scope
                    }

                    // Close button for tabs (except if it's the only one)
                    if self.available_tabs.len() > 1 {
                        ui.same_line();
                        let should_remove = {
                            let _close_token = ui.push_style_color(StyleColor::Button, [0.6, 0.2, 0.2, 0.8]);
                            ui.small_button(&format!("×##{}", i))
                            // _close_token automatically dropped here
                        };

                        if should_remove {
                            // Remove this tab
                            if is_active && self.available_tabs.len() > 1 {
                                // Switch to another tab before removing
                                let next_index = if i > 0 { i - 1 } else { 1 };
                                if next_index < self.available_tabs.len() {
                                    self.active_tab = self.available_tabs[next_index].clone();
                                }
                            }
                            self.available_tabs.remove(i);
                            return; // Exit early to avoid index issues
                        }
                    }
                }

                // Add new tab button - in its own scope
                {
                    ui.same_line();
                    let _add_button_token = ui.push_style_color(StyleColor::Button, PulsarTheme::BLUE_PRIMARY);
                    let _add_hover_token = ui.push_style_color(StyleColor::ButtonHovered, PulsarTheme::BLUE_HOVER);

                    if ui.button_with_size("+ Add Tab", [100.0, 28.0]) {
                        self.show_tab_search = true;
                        self.tab_search_query.clear();
                    }

                    if ui.is_item_hovered() {
                        ui.tooltip_text("Add new editor tab (Ctrl+T)");
                    }
                    // Tokens automatically dropped at end of scope
                }
            });
    }

    fn render_tab_search_modal(&mut self, ui: &Ui) {
        // Create a modal popup with proper visibility
        let center = ui.io().display_size;

        // Draw a semi-transparent overlay behind the modal
        let draw_list = ui.get_background_draw_list();
        draw_list
            .add_rect([0.0, 0.0], center, [0.0, 0.0, 0.0, 0.7])
            .filled(true)
            .build();

        // Modal window with higher z-order
        ui.window("Add Editor Tab")
            .position([center[0] * 0.5 - 300.0, center[1] * 0.5 - 250.0], Condition::Always)
            .size([600.0, 500.0], Condition::Always)
            .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE | WindowFlags::ALWAYS_AUTO_RESIZE)
            .focus_on_appearing(true)
            .build(|| {
                // Title with larger text
                {
                    let _title_color = ui.push_style_color(StyleColor::Text, PulsarTheme::BLUE_PRIMARY);
                    ui.text("🎯 Choose Editor to Add");
                }
                ui.separator();
                ui.spacing();

                // Search input with focus
                ui.text("Search:");
                if ui.input_text("##search", &mut self.tab_search_query)
                    .auto_select_all(true)
                    .build()
                {
                    // Auto focus on first load
                }
                ui.spacing();

                // Filter available tabs
                let all_tabs = EditorTab::all_tabs();
                let filtered_tabs: Vec<EditorTab> = all_tabs
                    .into_iter()
                    .filter(|tab| {
                        // Don't show tabs that are already open
                        !self.available_tabs.contains(tab) &&
                        // Filter by search query
                        (self.tab_search_query.is_empty() ||
                         tab.display_name().to_lowercase().contains(&self.tab_search_query.to_lowercase()) ||
                         tab.description().to_lowercase().contains(&self.tab_search_query.to_lowercase()))
                    })
                    .collect();

                // Scrollable list of tabs with better styling
                ui.child_window("TabList")
                    .size([0.0, -80.0])
                    .border(true)
                    .build(|| {
                        for tab in &filtered_tabs {
                            let label = format!("{} {}", tab.icon(), tab.display_name());

                            {
                                let _button_color = ui.push_style_color(StyleColor::Button, PulsarTheme::DARKER_PANEL);
                                let _hover_color = ui.push_style_color(StyleColor::ButtonHovered, PulsarTheme::BLUE_PRIMARY);

                                if ui.button_with_size(&label, [550.0, 45.0]) {
                                    self.available_tabs.push(tab.clone());
                                    self.active_tab = tab.clone();
                                    self.show_tab_search = false;
                                    return;
                                }
                            }

                            // Show description below
                            ui.text_colored(PulsarTheme::TEXT_MUTED, &format!("    {}", tab.description()));
                            ui.spacing();
                        }

                        if filtered_tabs.is_empty() {
                            ui.text_colored(PulsarTheme::TEXT_MUTED, "❌ No tabs available or all tabs already open");
                        }
                    });

                ui.separator();
                ui.spacing();

                // Bottom buttons
                {
                    let _cancel_color = ui.push_style_color(StyleColor::Button, [0.6, 0.2, 0.2, 1.0]);
                    if ui.button_with_size("Cancel", [80.0, 30.0]) {
                        self.show_tab_search = false;
                    }
                }
                ui.same_line();
                ui.text_colored(PulsarTheme::TEXT_MUTED, "💡 Tip: Use Ctrl+T to quickly add tabs");

                // Close on Escape key
                if ui.is_key_pressed(imgui::Key::Escape) {
                    self.show_tab_search = false;
                }
            });
    }

    fn render_active_tab_content(&self, ui: &Ui) {
        match self.active_tab {
            EditorTab::LevelEditor => self.render_level_editor_content(ui),
            EditorTab::ScriptEditor => self.render_script_editor_content(ui),
            EditorTab::BlueprintEditor => self.render_blueprint_editor_content(ui),
            EditorTab::MaterialEditor => self.render_material_editor_content(ui),
            EditorTab::AnimationEditor => self.render_animation_editor_content(ui),
            EditorTab::ParticleEditor => self.render_particle_editor_content(ui),
            EditorTab::AudioEditor => self.render_audio_editor_content(ui),
            EditorTab::TerrainEditor => self.render_terrain_editor_content(ui),
            EditorTab::PhysicsDebug => self.render_physics_debug_content(ui),
            EditorTab::Profiler => self.render_profiler_content(ui),
        }
    }

    // Additional tab content renderers
    fn render_script_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📜 Script Editor");
        ui.separator();

        ui.text("Write and debug game scripts in Rust");
        ui.spacing();

        // Code editor placeholder
        let mut code = String::from("// Pulsar Engine Script\nfn main() {\n    println!(\"Hello, Pulsar!\");\n}");
        ui.input_text_multiline("##code", &mut code, [0.0, 300.0]).build();
    }

    fn render_blueprint_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🔧 Blueprint Editor");
        ui.separator();

        ui.text("Visual scripting system for game logic");
        ui.spacing();

        // Node graph placeholder
        let pos = ui.cursor_screen_pos();
        let size = [600.0, 400.0];

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::PURE_BLACK)
            .filled(true)
            .build();

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::SUBTLE_BORDER)
            .build();

        ui.dummy(size);
    }

    fn render_material_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🎨 Material Editor");
        ui.separator();

        // Material properties
        let mut albedo = [1.0f32, 1.0, 1.0, 1.0];
        ui.color_edit4("Albedo", &mut albedo);

        let mut metallic = 0.0f32;
        ui.slider("Metallic", 0.0, 1.0, &mut metallic);

        let mut roughness = 0.5f32;
        ui.slider("Roughness", 0.0, 1.0, &mut roughness);
    }

    fn render_animation_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🎬 Animation Editor");
        ui.separator();

        // Timeline
        let mut timeline = 0.0f32;
        ui.slider("Timeline", 0.0, 10.0, &mut timeline);

        ui.spacing();
        ui.text("Animation Tracks:");
        ui.bullet_text("Walk Cycle");
        ui.bullet_text("Run Cycle");
        ui.bullet_text("Attack Animation");
    }

    fn render_particle_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "✨ Particle Editor");
        ui.separator();

        let mut emission_rate = 100.0f32;
        ui.slider("Emission Rate", 1.0, 1000.0, &mut emission_rate);

        let mut lifetime = 2.0f32;
        ui.slider("Particle Lifetime", 0.1, 10.0, &mut lifetime);
    }

    fn render_audio_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🔊 Audio Editor");
        ui.separator();

        let mut master_volume = 0.8f32;
        ui.slider("Master Volume", 0.0, 1.0, &mut master_volume);

        let mut music_volume = 0.6f32;
        ui.slider("Music Volume", 0.0, 1.0, &mut music_volume);
    }

    fn render_terrain_editor_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🏔️ Terrain Editor");
        ui.separator();

        let mut brush_size = 10.0f32;
        ui.slider("Brush Size", 1.0, 50.0, &mut brush_size);

        let mut strength = 0.5f32;
        ui.slider("Strength", 0.0, 1.0, &mut strength);
    }

    fn render_physics_debug_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "⚡ Physics Debug");
        ui.separator();

        ui.checkbox("Show Colliders", &mut true);
        ui.checkbox("Show Velocities", &mut false);
        ui.checkbox("Show Forces", &mut false);
    }

    fn render_profiler_content(&self, ui: &Ui) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📊 Profiler");
        ui.separator();

        ui.text("CPU Usage: 45%");
        ProgressBar::new(0.45).build(ui);

        ui.text("GPU Usage: 62%");
        ProgressBar::new(0.62).build(ui);

        ui.text("Memory: 1.2GB / 8.0GB");
        ProgressBar::new(0.15).build(ui);
    }
}

impl SimpleGameUI {
    /// Update panel sizes based on window size for responsive resizing
    fn update_responsive_panel_sizes(&mut self, available_width: f32, available_height: f32) {
        // Calculate responsive panel sizes based on screen size
        let base_left_width = (available_width * 0.18).max(150.0).min(available_width * 0.35);
        let base_right_width = (available_width * 0.22).max(200.0).min(available_width * 0.35);
        let base_bottom_height = (available_height * 0.25).max(100.0).min(available_height * 0.45);

        // Smoothly transition to new sizes for better performance
        let transition_speed = 0.1;
        self.left_panel_width = self.left_panel_width + (base_left_width - self.left_panel_width) * transition_speed;
        self.right_panel_width = self.right_panel_width + (base_right_width - self.right_panel_width) * transition_speed;
        self.bottom_panel_height = self.bottom_panel_height + (base_bottom_height - self.bottom_panel_height) * transition_speed;

        // Ensure minimum and maximum constraints
        self.left_panel_width = self.left_panel_width.max(150.0).min(available_width * 0.4);
        self.right_panel_width = self.right_panel_width.max(200.0).min(available_width * 0.4);
        self.bottom_panel_height = self.bottom_panel_height.max(100.0).min(available_height * 0.5);
    }
}

impl Default for SimpleGameUI {
    fn default() -> Self {
        Self::new()
    }
}