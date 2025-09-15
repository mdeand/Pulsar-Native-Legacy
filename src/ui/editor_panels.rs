use imgui::*;
use crate::ui::{PulsarTheme, VisualEffects, ButtonVariant};

/// Different types of editor panels available
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorPanelType {
    LevelEditor,
    ScriptEditor,
    BlueprintEditor,
    AssetBrowser,
    Inspector,
    Hierarchy,
    Console,
    Profiler,
    Animation,
    Material,
    Particle,
    Audio,
    Physics,
    Lighting,
    Terrain,
    Weather,
}

impl EditorPanelType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::LevelEditor => "Level Editor",
            Self::ScriptEditor => "Script Editor",
            Self::BlueprintEditor => "Blueprint Editor",
            Self::AssetBrowser => "Asset Browser",
            Self::Inspector => "Inspector",
            Self::Hierarchy => "Hierarchy",
            Self::Console => "Console",
            Self::Profiler => "Profiler",
            Self::Animation => "Animation",
            Self::Material => "Material Editor",
            Self::Particle => "Particle System",
            Self::Audio => "Audio Mixer",
            Self::Physics => "Physics",
            Self::Lighting => "Lighting",
            Self::Terrain => "Terrain Editor",
            Self::Weather => "Weather System",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::LevelEditor => "🌍",
            Self::ScriptEditor => "📜",
            Self::BlueprintEditor => "🔧",
            Self::AssetBrowser => "📁",
            Self::Inspector => "🔍",
            Self::Hierarchy => "📋",
            Self::Console => "💻",
            Self::Profiler => "📊",
            Self::Animation => "🎬",
            Self::Material => "🎨",
            Self::Particle => "✨",
            Self::Audio => "🔊",
            Self::Physics => "⚡",
            Self::Lighting => "💡",
            Self::Terrain => "🏔️",
            Self::Weather => "🌦️",
        }
    }

    pub fn all_panels() -> Vec<Self> {
        vec![
            Self::LevelEditor, Self::ScriptEditor, Self::BlueprintEditor,
            Self::AssetBrowser, Self::Inspector, Self::Hierarchy,
            Self::Console, Self::Profiler, Self::Animation,
            Self::Material, Self::Particle, Self::Audio,
            Self::Physics, Self::Lighting, Self::Terrain, Self::Weather,
        ]
    }
}

/// Content renderer for each editor panel type
pub struct EditorPanelRenderer;

impl EditorPanelRenderer {
    pub fn render_panel_content(ui: &Ui, panel_type: EditorPanelType, content_region: [f32; 2]) {
        match panel_type {
            EditorPanelType::LevelEditor => Self::render_level_editor(ui, content_region),
            EditorPanelType::ScriptEditor => Self::render_script_editor(ui, content_region),
            EditorPanelType::BlueprintEditor => Self::render_blueprint_editor(ui, content_region),
            EditorPanelType::AssetBrowser => Self::render_asset_browser(ui, content_region),
            EditorPanelType::Inspector => Self::render_inspector(ui, content_region),
            EditorPanelType::Hierarchy => Self::render_hierarchy(ui, content_region),
            EditorPanelType::Console => Self::render_console(ui, content_region),
            EditorPanelType::Profiler => Self::render_profiler(ui, content_region),
            EditorPanelType::Animation => Self::render_animation(ui, content_region),
            EditorPanelType::Material => Self::render_material_editor(ui, content_region),
            EditorPanelType::Particle => Self::render_particle_system(ui, content_region),
            EditorPanelType::Audio => Self::render_audio_mixer(ui, content_region),
            EditorPanelType::Physics => Self::render_physics(ui, content_region),
            EditorPanelType::Lighting => Self::render_lighting(ui, content_region),
            EditorPanelType::Terrain => Self::render_terrain_editor(ui, content_region),
            EditorPanelType::Weather => Self::render_weather_system(ui, content_region),
        }
    }

    fn render_level_editor(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🌍 Level Editor");
        ui.separator();

        // Toolbar
        if ui.button_with_size("New Level", [80.0, 24.0]) {}
        ui.same_line();
        if ui.button_with_size("Load", [60.0, 24.0]) {}
        ui.same_line();
        if ui.button_with_size("Save", [60.0, 24.0]) {}

        ui.spacing();

        // 3D Viewport placeholder
        let pos = ui.cursor_screen_pos();
        let size = [content_region[0] - 20.0, content_region[1] - 100.0];

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::DARKER_PANEL)
            .filled(true)
            .build();

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::SUBTLE_BORDER)
            .build();

        // Viewport controls overlay
        let center = [pos[0] + size[0] / 2.0, pos[1] + size[1] / 2.0];
        ui.get_window_draw_list()
            .add_text(center, PulsarTheme::TEXT_MUTED, "3D Viewport - Click to focus");

        ui.dummy(size);
    }

    fn render_script_editor(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📜 Script Editor");
        ui.separator();

        // File tabs
        if ui.button("main.rs") {}
        ui.same_line();
        if ui.button("+ New Script") {}

        ui.spacing();

        // Code editor area
        let mut code = String::from("// Pulsar Engine Script\nfn main() {\n    println!(\"Hello, Pulsar!\");\n}");
        ui.input_text_multiline("##code", &mut code, [content_region[0] - 20.0, content_region[1] - 80.0])
            .build();
    }

    fn render_blueprint_editor(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🔧 Blueprint Editor");
        ui.separator();

        // Node graph area
        let pos = ui.cursor_screen_pos();
        let size = [content_region[0] - 20.0, content_region[1] - 60.0];

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::PURE_BLACK)
            .filled(true)
            .build();

        // Grid pattern
        let grid_size = 20.0;
        for x in (0..(size[0] as i32)).step_by(grid_size as usize) {
            ui.get_window_draw_list()
                .add_line([pos[0] + x as f32, pos[1]], [pos[0] + x as f32, pos[1] + size[1]], PulsarTheme::SUBTLE_BORDER)
                .build();
        }
        for y in (0..(size[1] as i32)).step_by(grid_size as usize) {
            ui.get_window_draw_list()
                .add_line([pos[0], pos[1] + y as f32], [pos[0] + size[0], pos[1] + y as f32], PulsarTheme::SUBTLE_BORDER)
                .build();
        }

        ui.dummy(size);
    }

    fn render_asset_browser(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📁 Asset Browser");
        ui.separator();

        // Search bar
        let mut search = String::new();
        ui.input_text("Search", &mut search).build();

        ui.spacing();

        // Asset grid
        let child_size = [content_region[0] - 20.0, content_region[1] - 80.0];
        ui.child_window("asset_grid")
            .size(child_size)
            .build(|| {
                let assets = vec!["texture1.png", "model.fbx", "audio.wav", "script.rs", "material.mat"];
                for (i, asset) in assets.iter().enumerate() {
                    if i > 0 && i % 3 != 0 { ui.same_line(); }

                    let pos = ui.cursor_screen_pos();
                    let thumbnail_size = [64.0, 64.0];

                    // Asset thumbnail
                    ui.get_window_draw_list()
                        .add_rect(pos, [pos[0] + thumbnail_size[0], pos[1] + thumbnail_size[1]], PulsarTheme::DARK_PANEL)
                        .filled(true)
                        .build();

                    ui.dummy(thumbnail_size);
                    ui.text(asset);
                    ui.spacing();
                }
            });
    }

    fn render_inspector(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🔍 Inspector");
        ui.separator();

        ui.text("Selected: None");
        ui.spacing();

        // Transform component
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

        // Mesh Renderer component
        if ui.collapsing_header("Mesh Renderer", TreeNodeFlags::empty()) {
            ui.text("Material: Default");
            if ui.button("Browse...") {}
        }
    }

    fn render_hierarchy(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📋 Hierarchy");
        ui.separator();

        // Scene tree
        if let Some(_token) = ui.tree_node("Scene") {
            if let Some(_token) = ui.tree_node("Player") {
                ui.tree_node_leaf("Mesh");
                ui.tree_node_leaf("Collider");
                ui.tree_node_leaf("Script");
            }
            if let Some(_token) = ui.tree_node("Environment") {
                ui.tree_node_leaf("Terrain");
                ui.tree_node_leaf("Sky");
                if let Some(_token) = ui.tree_node("Lighting") {
                    ui.tree_node_leaf("Sun");
                    ui.tree_node_leaf("Ambient");
                }
            }
        }
    }

    fn render_console(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "💻 Console");
        ui.separator();

        // Console output
        let child_size = [content_region[0] - 20.0, content_region[1] - 100.0];
        ui.child_window("console_output")
            .size(child_size)
            .build(|| {
                ui.text_colored([0.5, 1.0, 0.5, 1.0], "[INFO] Pulsar Engine initialized");
                ui.text_colored([1.0, 1.0, 0.5, 1.0], "[WARN] Missing texture: default.png");
                ui.text_colored([1.0, 0.5, 0.5, 1.0], "[ERROR] Failed to load script: test.rs");
                ui.text_colored([0.5, 1.0, 0.5, 1.0], "[INFO] Level loaded successfully");
            });

        // Command input
        let mut command = String::new();
        ui.input_text("Command", &mut command).build();
    }

    fn render_profiler(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "📊 Profiler");
        ui.separator();

        // Performance metrics
        ui.text("FPS: 60");
        ui.text("Frame Time: 16.67ms");
        ui.text("Draw Calls: 245");
        ui.text("Triangles: 45,231");

        ui.spacing();

        // Performance graph placeholder
        let pos = ui.cursor_screen_pos();
        let size = [content_region[0] - 20.0, 100.0];

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::DARKER_PANEL)
            .filled(true)
            .build();

        ui.dummy(size);
    }

    fn render_animation(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🎬 Animation");
        ui.separator();

        // Timeline
        let timeline_height = 60.0;
        let pos = ui.cursor_screen_pos();
        let size = [content_region[0] - 20.0, timeline_height];

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::DARK_PANEL)
            .filled(true)
            .build();

        ui.dummy(size);

        // Animation controls
        if ui.button("⏮") {}
        ui.same_line();
        if ui.button("▶") {}
        ui.same_line();
        if ui.button("⏸") {}
        ui.same_line();
        if ui.button("⏭") {}
    }

    fn render_material_editor(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🎨 Material Editor");
        ui.separator();

        ui.text("Material: Default");
        ui.spacing();

        // Material properties
        let mut albedo = [1.0f32, 1.0, 1.0, 1.0];
        ui.color_edit4("Albedo", &mut albedo);

        let mut metallic = 0.0f32;
        ui.slider("Metallic", 0.0, 1.0, &mut metallic);

        let mut roughness = 0.5f32;
        ui.slider("Roughness", 0.0, 1.0, &mut roughness);
    }

    fn render_particle_system(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "✨ Particle System");
        ui.separator();

        let mut emission_rate = 100.0f32;
        ui.slider("Emission Rate", 1.0, 1000.0, &mut emission_rate);

        let mut lifetime = 2.0f32;
        ui.slider("Particle Lifetime", 0.1, 10.0, &mut lifetime);

        let mut size = 1.0f32;
        ui.slider("Size", 0.1, 10.0, &mut size);
    }

    fn render_audio_mixer(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🔊 Audio Mixer");
        ui.separator();

        ui.text("Master Volume");
        let mut master_vol = 0.8f32;
        ui.slider("##master", 0.0, 1.0, &mut master_vol);

        ui.text("Music Volume");
        let mut music_vol = 0.6f32;
        ui.slider("##music", 0.0, 1.0, &mut music_vol);

        ui.text("SFX Volume");
        let mut sfx_vol = 0.9f32;
        ui.slider("##sfx", 0.0, 1.0, &mut sfx_vol);
    }

    fn render_physics(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "⚡ Physics");
        ui.separator();

        let mut gravity = -9.81f32;
        ui.input_float("Gravity", &mut gravity).build();

        let mut time_scale = 1.0f32;
        ui.slider("Time Scale", 0.0, 2.0, &mut time_scale);

        ui.checkbox("Debug Draw", &mut false);
    }

    fn render_lighting(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "💡 Lighting");
        ui.separator();

        ui.text("Sun Light");
        let mut sun_color = [1.0f32, 0.95, 0.8, 1.0];
        ui.color_edit3("Color", &mut sun_color);

        let mut sun_intensity = 1.0f32;
        ui.slider("Intensity", 0.0, 5.0, &mut sun_intensity);

        ui.spacing();
        ui.text("Ambient");
        let mut ambient = [0.2f32, 0.2, 0.3, 1.0];
        ui.color_edit3("Ambient Color", &mut ambient);
    }

    fn render_terrain_editor(ui: &Ui, content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🏔️ Terrain Editor");
        ui.separator();

        // Heightmap view
        let pos = ui.cursor_screen_pos();
        let size = [content_region[0] - 20.0, 200.0];

        ui.get_window_draw_list()
            .add_rect(pos, [pos[0] + size[0], pos[1] + size[1]], PulsarTheme::DARKER_PANEL)
            .filled(true)
            .build();

        ui.dummy(size);

        // Terrain tools
        ui.text("Brush Size:");
        let mut brush_size = 10.0f32;
        ui.slider("##brush_size", 1.0, 50.0, &mut brush_size);

        ui.text("Strength:");
        let mut strength = 0.5f32;
        ui.slider("##strength", 0.0, 1.0, &mut strength);
    }

    fn render_weather_system(ui: &Ui, _content_region: [f32; 2]) {
        ui.text_colored(PulsarTheme::TEXT_PRIMARY, "🌦️ Weather System");
        ui.separator();

        ui.text("Weather Type:");
        let weather_types = ["Clear", "Cloudy", "Rain", "Storm", "Snow", "Fog"];
        let mut current_weather = 0;
        ui.combo("##weather", &mut current_weather, &weather_types, |item| item.into());

        ui.text("Intensity:");
        let mut intensity = 0.5f32;
        ui.slider("##intensity", 0.0, 1.0, &mut intensity);

        ui.text("Wind Speed:");
        let mut wind_speed = 5.0f32;
        ui.slider("##wind", 0.0, 50.0, &mut wind_speed);
    }
}