use imgui::*;

pub struct GameEngineUI {
    // Editor state
    active_editor: String,
    available_editors: Vec<EditorInfo>,

    // Panel visibility
    show_scene_hierarchy: bool,
    show_properties: bool,
    show_asset_browser: bool,
    show_console: bool,
    show_profiler: bool,

    // Tab management
    open_tabs: Vec<EditorTab>,
    next_tab_id: usize,

    // Search state
    search_query: String,
    show_editor_search: bool,

    // UI state
    main_menu_height: f32,
}

#[derive(Clone)]
pub struct EditorInfo {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub description: String,
}

#[derive(Clone)]
pub struct EditorTab {
    pub id: usize,
    pub editor_id: String,
    pub title: String,
    pub icon: String,
    pub is_modified: bool,
}

impl GameEngineUI {
    pub fn new() -> Self {
        let editors = vec![
            EditorInfo {
                id: "level".to_string(),
                title: "Level Editor".to_string(),
                icon: "🗺️".to_string(),
                description: "Design and build game levels".to_string(),
            },
            EditorInfo {
                id: "script".to_string(),
                title: "Script Editor".to_string(),
                icon: "📜".to_string(),
                description: "Write and debug game scripts".to_string(),
            },
            EditorInfo {
                id: "blueprint".to_string(),
                title: "Blueprint Editor".to_string(),
                icon: "🔧".to_string(),
                description: "Visual scripting system".to_string(),
            },
            EditorInfo {
                id: "material".to_string(),
                title: "Material Editor".to_string(),
                icon: "🎨".to_string(),
                description: "Create and edit materials".to_string(),
            },
            EditorInfo {
                id: "animation".to_string(),
                title: "Animation Editor".to_string(),
                icon: "🎭".to_string(),
                description: "Create character animations".to_string(),
            },
            EditorInfo {
                id: "particle".to_string(),
                title: "Particle System".to_string(),
                icon: "✨".to_string(),
                description: "Design particle effects".to_string(),
            },
            EditorInfo {
                id: "sound".to_string(),
                title: "Sound Editor".to_string(),
                icon: "🎵".to_string(),
                description: "Edit and manage audio".to_string(),
            },
            EditorInfo {
                id: "terrain".to_string(),
                title: "Terrain Editor".to_string(),
                icon: "🏔️".to_string(),
                description: "Sculpt and paint terrain".to_string(),
            },
            EditorInfo {
                id: "navmesh".to_string(),
                title: "Navigation Mesh".to_string(),
                icon: "🧭".to_string(),
                description: "AI pathfinding setup".to_string(),
            },
            EditorInfo {
                id: "physics".to_string(),
                title: "Physics Debug".to_string(),
                icon: "⚡".to_string(),
                description: "Debug physics simulation".to_string(),
            },
        ];

        let mut ui = Self {
            active_editor: "level".to_string(),
            available_editors: editors,
            show_scene_hierarchy: true,
            show_properties: true,
            show_asset_browser: true,
            show_console: true,
            show_profiler: false,
            open_tabs: Vec::new(),
            next_tab_id: 0,
            search_query: String::new(),
            show_editor_search: false,
            main_menu_height: 20.0,
        };

        // Add default tabs
        ui.add_tab("level");
        ui.add_tab("script");

        ui
    }

    pub fn render(&mut self, ui: &Ui, fps: u64) {
        // Main menu bar
        self.render_main_menu_bar(ui, fps);

        // Main layout - use the full window
        let window_size = ui.io().display_size;
        let menu_height = ui.frame_height();

        // Tab bar
        ui.window("TabBar")
            .position([0.0, menu_height], Condition::Always)
            .size([window_size[0], 30.0], Condition::Always)
            .flags(WindowFlags::NO_DECORATION | WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE)
            .build(|| {
                self.render_tab_bar(ui);
            });

        // Main content area
        let content_y = menu_height + 30.0;
        let content_height = window_size[1] - content_y;

        // Side panels and main editor in columns
        ui.window("MainContent")
            .position([0.0, content_y], Condition::Always)
            .size([window_size[0], content_height], Condition::Always)
            .flags(WindowFlags::NO_DECORATION | WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE)
            .build(|| {
                // Create a 3-column layout: left panel | main content | right panel
                ui.columns(3, "MainLayout", true);

                // Left column - Scene Hierarchy
                ui.set_column_width(0, 250.0);
                if self.show_scene_hierarchy {
                    self.render_scene_hierarchy(ui);
                }

                ui.next_column();

                // Middle column - Main Editor
                ui.set_column_width(1, window_size[0] - 550.0); // Total - left - right
                self.render_active_editor(ui);

                ui.next_column();

                // Right column - Properties and Asset Browser
                ui.set_column_width(2, 300.0);
                if self.show_properties {
                    self.render_properties_panel(ui);
                }

                if self.show_asset_browser {
                    self.render_asset_browser(ui);
                }

                ui.columns(1, "", false); // Reset columns
            });

        // Bottom panels
        if self.show_console {
            ui.window("ConsoleWindow")
                .position([0.0, window_size[1] - 200.0], Condition::Always)
                .size([window_size[0], 200.0], Condition::Always)
                .flags(WindowFlags::NO_DECORATION | WindowFlags::NO_MOVE)
                .build(|| {
                    self.render_console(ui);
                });
        }

        // Editor search modal
        if self.show_editor_search {
            self.render_editor_search_modal(ui);
        }
    }

    fn render_main_menu_bar(&mut self, ui: &Ui, fps: u64) {
        if let Some(_menu_bar) = ui.begin_main_menu_bar() {
            // File menu
            if let Some(_file_menu) = ui.begin_menu("File") {
                if ui.menu_item("New Project") {}
                if ui.menu_item("Open Project") {}
                if ui.menu_item("Save Project") {}
                ui.separator();
                if ui.menu_item("Import Asset") {}
                if ui.menu_item("Export Project") {}
                ui.separator();
                if ui.menu_item("Exit") {
                    std::process::exit(0);
                }
            }

            // Edit menu
            if let Some(_edit_menu) = ui.begin_menu("Edit") {
                if ui.menu_item("Undo") {}
                if ui.menu_item("Redo") {}
                ui.separator();
                if ui.menu_item("Project Settings") {}
                if ui.menu_item("Preferences") {}
            }

            // View menu
            if let Some(_view_menu) = ui.begin_menu("View") {
                if ui.menu_item_config("Scene Hierarchy")
                    .selected(self.show_scene_hierarchy)
                    .build()
                {
                    self.show_scene_hierarchy = !self.show_scene_hierarchy;
                }
                if ui.menu_item_config("Properties")
                    .selected(self.show_properties)
                    .build()
                {
                    self.show_properties = !self.show_properties;
                }
                if ui.menu_item_config("Asset Browser")
                    .selected(self.show_asset_browser)
                    .build()
                {
                    self.show_asset_browser = !self.show_asset_browser;
                }
                if ui.menu_item_config("Console")
                    .selected(self.show_console)
                    .build()
                {
                    self.show_console = !self.show_console;
                }
                if ui.menu_item_config("Profiler")
                    .selected(self.show_profiler)
                    .build()
                {
                    self.show_profiler = !self.show_profiler;
                }
            }

            // Tools menu
            if let Some(_tools_menu) = ui.begin_menu("Tools") {
                if ui.menu_item("Package Manager") {}
                if ui.menu_item("Version Control") {}
                if ui.menu_item("Build Settings") {}
            }

            // Help menu
            if let Some(_help_menu) = ui.begin_menu("Help") {
                if ui.menu_item("Documentation") {}
                if ui.menu_item("Tutorials") {}
                if ui.menu_item("About") {}
            }

            // Performance info on the right
            let fps_text = format!("FPS: {} | Frame: {:.2}ms", fps, 1000.0 / fps.max(1) as f32);
            let text_size = ui.calc_text_size(&fps_text);
            let menu_width = ui.io().display_size[0];
            ui.set_cursor_pos([menu_width - text_size[0] - 16.0, 4.0]);
            ui.text_colored([0.7, 0.7, 0.7, 1.0], &fps_text);
        }
    }

    fn render_tab_bar(&mut self, ui: &Ui) {
        // Simple tab bar implementation for imgui 0.10.0
        for (index, tab) in self.open_tabs.iter().enumerate() {
            let is_active = self.active_editor == tab.editor_id;

            if index > 0 {
                ui.same_line();
            }

            let tab_label = format!("{} {}", tab.icon, tab.title);

            // Tab button with different style for active tab
            let style_token = if is_active {
                Some(ui.push_style_color(StyleColor::Button, [0.0, 0.3, 0.8, 1.0]))
            } else {
                Some(ui.push_style_color(StyleColor::Button, [0.1, 0.1, 0.1, 1.0]))
            };

            if ui.button(&tab_label) {
                self.active_editor = tab.editor_id.clone();
            }

            if let Some(token) = style_token {
                token.pop();
            }

            // Close button for tabs (except if it's the only one)
            if self.open_tabs.len() > 1 {
                ui.same_line();
                let close_id = format!("×##{}", index);
                if ui.small_button(&close_id) {
                    // Mark for removal (we'll handle this outside the loop)
                    // For now, just change to another tab if this was active
                    if is_active && self.open_tabs.len() > 1 {
                        let next_index = if index > 0 { index - 1 } else { 1 };
                        if next_index < self.open_tabs.len() {
                            self.active_editor = self.open_tabs[next_index].editor_id.clone();
                        }
                    }
                    self.open_tabs.remove(index);
                    return; // Exit early to avoid index issues
                }
            }
        }

        // Add new tab button
        ui.same_line();
        if ui.button("+") {
            self.show_editor_search = true;
            self.search_query.clear();
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Add new editor tab");
        }
    }

    fn render_editor_search_modal(&mut self, ui: &Ui) {
        ui.open_popup("Add Editor");

        let center = ui.io().display_size;
        ui.window("Add Editor")
            .position([center[0] * 0.5 - 250.0, center[1] * 0.5 - 200.0], Condition::Always)
            .size([500.0, 400.0], Condition::Always)
            .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE)
            .build(|| {
                ui.text("Search for editor to add:");

                if ui.input_text("##search", &mut self.search_query).build() {
                    // Search changed
                }

                ui.separator();

                // Filter editors
                let filtered_editors: Vec<EditorInfo> = self.available_editors
                    .iter()
                    .filter(|editor| {
                        self.search_query.is_empty() ||
                        editor.title.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                        editor.description.to_lowercase().contains(&self.search_query.to_lowercase())
                    })
                    .cloned()
                    .collect();

                ui.child_window("EditorList")
                    .size([0.0, -32.0])
                    .build(|| {
                        for editor in &filtered_editors {
                            if ui.selectable(&format!("{} {}", editor.icon, editor.title)) {
                                self.add_tab(&editor.id);
                                self.show_editor_search = false;
                                return;
                            }

                            ui.text_disabled(&format!("    {}", editor.description));
                        }
                    });

                ui.separator();
                if ui.button("Cancel") {
                    self.show_editor_search = false;
                }
            });
    }

    fn render_active_editor(&self, ui: &Ui) {
        ui.window("Editor")
            .flags(WindowFlags::NO_COLLAPSE)
            .build(|| {
                match self.active_editor.as_str() {
                    "level" => self.render_level_editor(ui),
                    "script" => self.render_script_editor(ui),
                    "blueprint" => self.render_blueprint_editor(ui),
                    "material" => self.render_material_editor(ui),
                    "animation" => self.render_animation_editor(ui),
                    "particle" => self.render_particle_editor(ui),
                    "sound" => self.render_sound_editor(ui),
                    "terrain" => self.render_terrain_editor(ui),
                    "navmesh" => self.render_navmesh_editor(ui),
                    "physics" => self.render_physics_editor(ui),
                    _ => {
                        ui.text("Unknown editor");
                    }
                }
            });
    }

    fn render_scene_hierarchy(&self, ui: &Ui) {
        ui.window("Scene Hierarchy")
            .flags(WindowFlags::NO_COLLAPSE)
            .size([250.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("🌳 Scene Objects");
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
                    if let Some(_audio_tree) = ui.tree_node("🎵 Audio") {
                        ui.text("🔊 AudioListener");
                        ui.text("🎼 BackgroundMusic");
                    }
                }
            });
    }

    fn render_properties_panel(&self, ui: &Ui) {
        ui.window("Properties")
            .flags(WindowFlags::NO_COLLAPSE)
            .size([300.0, 500.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("🔧 Object Properties");
                ui.separator();

                if ui.collapsing_header("Transform", TreeNodeFlags::DEFAULT_OPEN) {
                    let mut pos = [0.0, 0.0, 0.0];
                    ui.text("Position:");
                    ui.input_float3("##pos", &mut pos).build();

                    let mut rot = [0.0, 0.0, 0.0];
                    ui.text("Rotation:");
                    ui.input_float3("##rot", &mut rot).build();

                    let mut scale = [1.0, 1.0, 1.0];
                    ui.text("Scale:");
                    ui.input_float3("##scale", &mut scale).build();
                }

                if ui.collapsing_header("Material", TreeNodeFlags::empty()) {
                    let mut base_color = [1.0, 1.0, 1.0, 1.0];
                    ui.text("Base Color:");
                    ui.color_edit4("##basecolor", &mut base_color);

                    let mut metallic = 0.0;
                    ui.text("Metallic:");
                    ui.slider("##metallic", 0.0, 1.0, &mut metallic);

                    let mut roughness = 0.5;
                    ui.text("Roughness:");
                    ui.slider("##roughness", 0.0, 1.0, &mut roughness);
                }

                if ui.collapsing_header("Physics", TreeNodeFlags::empty()) {
                    let mut is_kinematic = false;
                    let mut use_gravity = true;
                    ui.checkbox("Is Kinematic", &mut is_kinematic);
                    ui.checkbox("Use Gravity", &mut use_gravity);

                    let mut mass = 1.0;
                    ui.text("Mass:");
                    ui.input_float("##mass", &mut mass).build();
                }
            });
    }

    fn render_asset_browser(&self, ui: &Ui) {
        ui.window("Asset Browser")
            .flags(WindowFlags::NO_COLLAPSE)
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("📁 Project Assets");
                ui.separator();

                // Asset categories
                let categories = ["All", "Meshes", "Textures", "Materials", "Scripts", "Audio", "Animations"];
                let mut selected_category = 0;

                for (i, category) in categories.iter().enumerate() {
                    if ui.selectable_config(category)
                        .selected(selected_category == i)
                        .flags(SelectableFlags::DONT_CLOSE_POPUPS)
                        .build()
                    {
                        selected_category = i;
                    }
                    ui.same_line();
                }
                ui.new_line();
                ui.separator();

                // Asset grid
                ui.child_window("AssetGrid").build(|| {
                    let assets = vec![
                        ("🎮", "PlayerModel.fbx"),
                        ("🏠", "House.fbx"),
                        ("🌳", "Tree.fbx"),
                        ("🎨", "Brick_Material.mat"),
                        ("🖼️", "Texture_Grass.png"),
                        ("🎵", "BGM_Adventure.ogg"),
                        ("📜", "PlayerScript.rs"),
                        ("✨", "MagicEffect.particle"),
                    ];

                    for (icon, name) in assets {
                        if ui.selectable(&format!("{} {}", icon, name)) {
                            // Asset selected
                        }
                    }
                });
            });
    }

    fn render_console(&self, ui: &Ui) {
        ui.window("Console")
            .flags(WindowFlags::NO_COLLAPSE)
            .size([600.0, 200.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("📋 Console Output");
                ui.separator();

                ui.child_window("ConsoleOutput")
                    .size([0.0, -25.0])
                    .build(|| {
                        ui.text_colored([0.8, 0.8, 0.8, 1.0], "[INFO] Game engine initialized");
                        ui.text_colored([0.9, 0.9, 0.4, 1.0], "[WARN] Asset 'missing_texture.png' not found");
                        ui.text_colored([0.4, 0.9, 0.4, 1.0], "[DEBUG] Physics simulation started");
                        ui.text_colored([0.9, 0.4, 0.4, 1.0], "[ERROR] Failed to load shader 'water.glsl'");
                        ui.text_colored([0.8, 0.8, 0.8, 1.0], "[INFO] Frame rate: 240 FPS");
                    });

                ui.separator();
                ui.input_text("Command", &mut String::new()).build();
            });
    }

    fn render_profiler(&self, ui: &Ui) {
        ui.window("Profiler")
            .flags(WindowFlags::NO_COLLAPSE)
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("📊 Performance Profiler");
                ui.separator();

                ui.text("CPU Usage: 45%");
                ProgressBar::new(0.45)
                    .size([0.0, 0.0])
                    .overlay_text("")
                    .build(ui);

                ui.text("GPU Usage: 62%");
                ProgressBar::new(0.62)
                    .size([0.0, 0.0])
                    .overlay_text("")
                    .build(ui);

                ui.text("Memory: 1.2GB / 8.0GB");
                ProgressBar::new(0.15)
                    .size([0.0, 0.0])
                    .overlay_text("")
                    .build(ui);

                ui.separator();
                ui.text("Frame Times (ms):");

                // Simplified frame time graph representation
                let frame_times = [4.1, 4.2, 4.0, 4.3, 4.1, 4.2, 4.4, 4.0];
                for (i, time) in frame_times.iter().enumerate() {
                    ui.text(&format!("Frame {}: {:.1}ms", i + 1, time));
                }
            });
    }

    // Individual editor rendering methods
    fn render_level_editor(&self, ui: &Ui) {
        ui.text("🗺️ Level Editor");
        ui.separator();

        ui.text("Design and build game levels with:");
        ui.bullet_text("Terrain sculpting tools");
        ui.bullet_text("Object placement system");
        ui.bullet_text("Lighting setup");
        ui.bullet_text("Collision detection");

        if ui.button("New Level") {
            // Create new level
        }
        ui.same_line();
        if ui.button("Load Level") {
            // Load existing level
        }
        ui.same_line();
        if ui.button("Save Level") {
            // Save current level
        }

        ui.separator();
        ui.text("Active Tools:");
        ui.radio_button("Select", &mut 0, 0);
        ui.same_line();
        ui.radio_button("Move", &mut 0, 1);
        ui.same_line();
        ui.radio_button("Rotate", &mut 0, 2);
        ui.same_line();
        ui.radio_button("Scale", &mut 0, 3);
    }

    fn render_script_editor(&self, ui: &Ui) {
        ui.text("📜 Script Editor");
        ui.separator();
        ui.text("Write and debug game scripts in Rust");

        ui.child_window("CodeEditor")
            .size([0.0, 200.0])
            .flags(WindowFlags::HORIZONTAL_SCROLLBAR)
            .build(|| {
                ui.text_colored([0.9, 0.7, 0.4, 1.0], "// Game script example");
                ui.text_colored([0.6, 0.8, 1.0, 1.0], "use");
                ui.same_line();
                ui.text(" pulsar_engine::*;");
                ui.text("");
                ui.text_colored([0.6, 0.8, 1.0, 1.0], "fn");
                ui.same_line();
                ui.text_colored([0.8, 1.0, 0.8, 1.0], " update");
                ui.same_line();
                ui.text("(world: &mut World) {");
                ui.text("    // Your game logic here");
                ui.text("}");
            });
    }

    fn render_blueprint_editor(&self, ui: &Ui) {
        ui.text("🔧 Blueprint Editor");
        ui.separator();
        ui.text("Visual scripting system for game logic");

        // Placeholder for node graph
        ui.child_window("NodeGraph").build(|| {
            ui.text("📋 Node Graph");
            ui.separator();
            ui.text("• Start Node");
            ui.text("• Input Handling");
            ui.text("• Movement Logic");
            ui.text("• Animation Trigger");
            ui.text("• End Node");
        });
    }

    fn render_material_editor(&self, ui: &Ui) {
        ui.text("🎨 Material Editor");
        ui.separator();
        ui.text("Create and edit materials for 3D objects");

        ui.text("Base Color:");
        ui.color_edit4("##basecolor", &mut [0.8, 0.8, 0.8, 1.0]);

        ui.text("Metallic:");
        let mut metallic = 0.0;
        ui.slider("##metallic", 0.0, 1.0, &mut metallic);

        let mut roughness = 0.5;
        ui.text("Roughness:");
        ui.slider("##roughness", 0.0, 1.0, &mut roughness);

        let mut normal = 1.0;
        ui.text("Normal Intensity:");
        ui.slider("##normal", 0.0, 2.0, &mut normal);
    }

    fn render_animation_editor(&self, ui: &Ui) {
        ui.text("🎭 Animation Editor");
        ui.separator();
        ui.text("Create character animations and cutscenes");

        let mut timeline = 0.0;
        ui.text("Animation Timeline:");
        ui.slider("##timeline", 0.0, 10.0, &mut timeline);

        ui.separator();
        ui.text("Animation Tracks:");
        ui.text("🏃 Walk Cycle");
        ui.text("🏃‍♂️ Run Cycle");
        ui.text("⚔️ Attack Animation");
        ui.text("🛡️ Block Animation");
    }

    fn render_particle_editor(&self, ui: &Ui) {
        ui.text("✨ Particle System");
        ui.separator();
        ui.text("Design visual effects and particle systems");

        let mut emission = 50.0;
        ui.text("Emission Rate:");
        ui.slider("##emission", 1.0, 1000.0, &mut emission);

        let mut life = 2.0;
        ui.text("Particle Life:");
        ui.slider("##life", 0.1, 10.0, &mut life);

        let mut start_size = 1.0;
        ui.text("Start Size:");
        ui.slider("##startsize", 0.1, 5.0, &mut start_size);

        let mut end_size = 0.1;
        ui.text("End Size:");
        ui.slider("##endsize", 0.0, 5.0, &mut end_size);
    }

    fn render_sound_editor(&self, ui: &Ui) {
        ui.text("🎵 Sound Editor");
        ui.separator();
        ui.text("Edit and manage audio assets");

        ui.text("🎼 Background Music");
        ui.text("🔊 Sound Effects");
        ui.text("🗣️ Voice Acting");
        ui.text("🔇 Audio Processing");

        ui.separator();
        let mut volume = 0.8;
        ui.text("Volume:");
        ui.slider("##volume", 0.0, 1.0, &mut volume);

        let mut pitch = 1.0;
        ui.text("Pitch:");
        ui.slider("##pitch", 0.5, 2.0, &mut pitch);
    }

    fn render_terrain_editor(&self, ui: &Ui) {
        ui.text("🏔️ Terrain Editor");
        ui.separator();
        ui.text("Sculpt and paint terrain for your world");

        let mut brush_size = 10.0;
        ui.text("Brush Size:");
        ui.slider("##brushsize", 1.0, 100.0, &mut brush_size);

        let mut strength = 0.5;
        ui.text("Brush Strength:");
        ui.slider("##strength", 0.1, 2.0, &mut strength);

        ui.separator();
        ui.text("Terrain Tools:");
        ui.button("🏔️ Raise");
        ui.same_line();
        ui.button("🕳️ Lower");
        ui.same_line();
        ui.button("🎨 Paint");
        ui.same_line();
        ui.button("🌿 Foliage");
    }

    fn render_navmesh_editor(&self, ui: &Ui) {
        ui.text("🧭 Navigation Mesh");
        ui.separator();
        ui.text("Setup AI pathfinding navigation");

        ui.button("Generate NavMesh");
        ui.same_line();
        ui.button("Clear NavMesh");

        ui.separator();
        ui.text("NavMesh Settings:");
        let mut radius = 0.5;
        ui.text("Agent Radius:");
        ui.slider("##radius", 0.1, 2.0, &mut radius);

        let mut height = 2.0;
        ui.text("Agent Height:");
        ui.slider("##height", 0.5, 5.0, &mut height);
    }

    fn render_physics_editor(&self, ui: &Ui) {
        ui.text("⚡ Physics Debug");
        ui.separator();
        ui.text("Debug and visualize physics simulation");

        ui.checkbox("Show Colliders", &mut true);
        ui.checkbox("Show Velocities", &mut false);
        ui.checkbox("Show Forces", &mut false);
        ui.checkbox("Show Joints", &mut true);

        ui.separator();
        ui.text("Physics Stats:");
        ui.text("Active Bodies: 147");
        ui.text("Collisions: 23");
        ui.text("Simulation Time: 2.3ms");
    }

    fn add_tab(&mut self, editor_id: &str) {
        // Check if tab already exists
        if self.open_tabs.iter().any(|tab| tab.editor_id == editor_id) {
            self.active_editor = editor_id.to_string();
            return;
        }

        if let Some(editor_info) = self.available_editors.iter().find(|e| e.id == editor_id) {
            let tab = EditorTab {
                id: self.next_tab_id,
                editor_id: editor_id.to_string(),
                title: editor_info.title.clone(),
                icon: editor_info.icon.clone(),
                is_modified: false,
            };

            self.open_tabs.push(tab);
            self.next_tab_id += 1;
            self.active_editor = editor_id.to_string();
        }
    }
}