use imgui::*;
use std::collections::HashMap;
use crate::ui::{PulsarTheme, VisualEffects, EditorPanelType};

/// Manages the overall layout and docking system for the Pulsar Engine UI
pub struct LayoutManager {
    pub dock_space_id: Id,
    pub main_menu_height: f32,
    pub status_bar_height: f32,
    pub sidebar_width: f32,
    pub sidebar_visible: bool,
    pub fullscreen_mode: bool,
    pub saved_layouts: HashMap<String, LayoutPreset>,
    pub current_layout: String,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            dock_space_id: Id::new("MainDockSpace"),
            main_menu_height: 24.0,
            status_bar_height: 20.0,
            sidebar_width: 250.0,
            sidebar_visible: true,
            fullscreen_mode: false,
            saved_layouts: Self::create_default_layouts(),
            current_layout: "Default".to_string(),
        }
    }

    /// Create default layout presets
    fn create_default_layouts() -> HashMap<String, LayoutPreset> {
        let mut layouts = HashMap::new();

        layouts.insert("Default".to_string(), LayoutPreset {
            name: "Default".to_string(),
            description: "Standard game engine layout".to_string(),
            panels: vec![
                PanelLayout { panel_type: EditorPanelType::LevelEditor, dock_area: DockArea::Center, size_ratio: 0.6 },
                PanelLayout { panel_type: EditorPanelType::Hierarchy, dock_area: DockArea::Left, size_ratio: 0.25 },
                PanelLayout { panel_type: EditorPanelType::Inspector, dock_area: DockArea::Right, size_ratio: 0.25 },
                PanelLayout { panel_type: EditorPanelType::AssetBrowser, dock_area: DockArea::Bottom, size_ratio: 0.3 },
                PanelLayout { panel_type: EditorPanelType::Console, dock_area: DockArea::Bottom, size_ratio: 0.3 },
            ],
        });

        layouts.insert("Code".to_string(), LayoutPreset {
            name: "Code".to_string(),
            description: "Focused on script editing".to_string(),
            panels: vec![
                PanelLayout { panel_type: EditorPanelType::ScriptEditor, dock_area: DockArea::Center, size_ratio: 0.7 },
                PanelLayout { panel_type: EditorPanelType::AssetBrowser, dock_area: DockArea::Left, size_ratio: 0.3 },
                PanelLayout { panel_type: EditorPanelType::Console, dock_area: DockArea::Bottom, size_ratio: 0.3 },
            ],
        });

        layouts.insert("Art".to_string(), LayoutPreset {
            name: "Art".to_string(),
            description: "Asset creation and material editing".to_string(),
            panels: vec![
                PanelLayout { panel_type: EditorPanelType::Material, dock_area: DockArea::Center, size_ratio: 0.5 },
                PanelLayout { panel_type: EditorPanelType::AssetBrowser, dock_area: DockArea::Left, size_ratio: 0.3 },
                PanelLayout { panel_type: EditorPanelType::Inspector, dock_area: DockArea::Right, size_ratio: 0.2 },
                PanelLayout { panel_type: EditorPanelType::Animation, dock_area: DockArea::Bottom, size_ratio: 0.3 },
            ],
        });

        layouts.insert("Debug".to_string(), LayoutPreset {
            name: "Debug".to_string(),
            description: "Performance monitoring and debugging".to_string(),
            panels: vec![
                PanelLayout { panel_type: EditorPanelType::LevelEditor, dock_area: DockArea::Center, size_ratio: 0.5 },
                PanelLayout { panel_type: EditorPanelType::Profiler, dock_area: DockArea::Right, size_ratio: 0.3 },
                PanelLayout { panel_type: EditorPanelType::Console, dock_area: DockArea::Bottom, size_ratio: 0.4 },
                PanelLayout { panel_type: EditorPanelType::Physics, dock_area: DockArea::Right, size_ratio: 0.2 },
            ],
        });

        layouts
    }

    /// Setup the main docking layout
    pub fn begin_dockspace(&self, ui: &Ui) {
        let viewport = ui.main_viewport();
        let work_pos = viewport.work_pos;
        let work_size = viewport.work_size;

        // Full window dockspace
        ui.set_next_window_pos(work_pos, Condition::Always);
        ui.set_next_window_size(work_size, Condition::Always);

        let window_flags = WindowFlags::NO_TITLE_BAR
            | WindowFlags::NO_COLLAPSE
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
            | WindowFlags::NO_NAV_FOCUS
            | WindowFlags::NO_BACKGROUND
            | WindowFlags::NO_DOCKING;

        ui.window("MainDockSpaceWindow")
            .flags(window_flags)
            .build(|| {
                // Create main dockspace
                let dockspace_flags = DockNodeFlags::PASSTHRU_CENTRAL_NODE;
                ui.dockspace_over_main_viewport_with_flags(self.dock_space_id, dockspace_flags);
            });
    }

    /// Render the main menu bar
    pub fn render_main_menu(&mut self, ui: &Ui) -> bool {
        let mut should_quit = false;

        ui.main_menu_bar(|| {
            // File menu
            ui.menu("File", || {
                if ui.menu_item("New Project") {}
                if ui.menu_item("Open Project") {}
                ui.separator();
                if ui.menu_item("New Scene") {}
                if ui.menu_item("Open Scene") {}
                if ui.menu_item("Save Scene") {}
                ui.separator();
                if ui.menu_item("Build Settings") {}
                ui.separator();
                if ui.menu_item("Exit") {
                    should_quit = true;
                }
            });

            // Edit menu
            ui.menu("Edit", || {
                if ui.menu_item("Undo") {}
                if ui.menu_item("Redo") {}
                ui.separator();
                if ui.menu_item("Cut") {}
                if ui.menu_item("Copy") {}
                if ui.menu_item("Paste") {}
                ui.separator();
                if ui.menu_item("Preferences") {}
            });

            // View menu
            ui.menu("View", || {
                ui.menu("Layout", || {
                    for layout_name in self.saved_layouts.keys() {
                        let is_current = layout_name == &self.current_layout;
                        if ui.menu_item_config(layout_name).selected(is_current).build() {
                            self.current_layout = layout_name.clone();
                        }
                    }
                    ui.separator();
                    if ui.menu_item("Save Layout As...") {}
                    if ui.menu_item("Reset Layout") {}
                });

                ui.separator();

                // Panel toggles
                ui.menu("Panels", || {
                    for panel_type in EditorPanelType::all_panels() {
                        let label = format!("{} {}", panel_type.icon(), panel_type.display_name());
                        if ui.menu_item(&label) {}
                    }
                });

                ui.separator();
                ui.menu_item_config("Sidebar").selected(self.sidebar_visible).build();
                ui.menu_item_config("Fullscreen").selected(self.fullscreen_mode).build();
            });

            // Tools menu
            ui.menu("Tools", || {
                if ui.menu_item("Asset Importer") {}
                if ui.menu_item("Build Tool") {}
                if ui.menu_item("Package Manager") {}
                ui.separator();
                if ui.menu_item("Performance Profiler") {}
                if ui.menu_item("Memory Analyzer") {}
            });

            // Window menu
            ui.menu("Window", || {
                if ui.menu_item("Minimize") {}
                if ui.menu_item("Maximize") {}
                ui.separator();
                if ui.menu_item("Reset Windows") {}
            });

            // Help menu
            ui.menu("Help", || {
                if ui.menu_item("Documentation") {}
                if ui.menu_item("Tutorials") {}
                if ui.menu_item("Community") {}
                ui.separator();
                if ui.menu_item("Report Bug") {}
                if ui.menu_item("Feature Request") {}
                ui.separator();
                if ui.menu_item("About Pulsar Engine") {}
            });

            // Right-aligned items
            let menu_bar_width = ui.content_region_avail()[0];
            let fps_text = format!("FPS: {}", crate::frame_counter::get_fps());
            let fps_width = ui.calc_text_size(&fps_text)[0];

            ui.set_cursor_pos_x(menu_bar_width - fps_width - 10.0);
            ui.text_colored(PulsarTheme::TEXT_SECONDARY, &fps_text);
        });

        should_quit
    }

    /// Render the status bar
    pub fn render_status_bar(&self, ui: &Ui) {
        let viewport = ui.main_viewport();
        let work_pos = viewport.work_pos;
        let work_size = viewport.work_size;

        // Position at bottom
        ui.set_next_window_pos([work_pos[0], work_pos[1] + work_size[1] - self.status_bar_height], Condition::Always);
        ui.set_next_window_size([work_size[0], self.status_bar_height], Condition::Always);

        let window_flags = WindowFlags::NO_TITLE_BAR
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_SCROLLBAR
            | WindowFlags::NO_COLLAPSE;

        ui.window("StatusBar")
            .flags(window_flags)
            .build(|| {
                // Status text
                ui.text_colored(PulsarTheme::TEXT_SECONDARY, "Ready");

                ui.same_line();
                ui.set_cursor_pos_x(200.0);
                ui.text_colored(PulsarTheme::TEXT_MUTED, "Scene: Untitled");

                // Right-aligned info
                let status_width = ui.content_region_avail()[0];
                let memory_text = "Memory: 256MB";
                let memory_width = ui.calc_text_size(memory_text)[0];

                ui.set_cursor_pos_x(status_width - memory_width - 10.0);
                ui.text_colored(PulsarTheme::TEXT_MUTED, memory_text);
            });
    }

    /// Apply a specific layout preset
    pub fn apply_layout(&mut self, layout_name: &str) {
        if let Some(layout) = self.saved_layouts.get(layout_name) {
            self.current_layout = layout_name.to_string();
            // Layout application would integrate with ImGui's docking system
            // This would need to be implemented with ImGui's dock builder API
        }
    }

    /// Get the current content area (excluding menu bar and status bar)
    pub fn get_content_area(&self, ui: &Ui) -> [f32; 4] {
        let viewport = ui.main_viewport();
        let work_pos = viewport.work_pos;
        let work_size = viewport.work_size;

        [
            work_pos[0],
            work_pos[1] + self.main_menu_height,
            work_size[0],
            work_size[1] - self.main_menu_height - self.status_bar_height,
        ]
    }

    /// Render floating panel hints for drag and drop
    pub fn render_drop_hints(&self, ui: &Ui, is_dragging: bool) {
        if !is_dragging {
            return;
        }

        let content_area = self.get_content_area(ui);
        let center_x = content_area[0] + content_area[2] / 2.0;
        let center_y = content_area[1] + content_area[3] / 2.0;

        // Central drop zone
        let drop_size = 100.0;
        let central_pos = [center_x - drop_size / 2.0, center_y - drop_size / 2.0];

        VisualEffects::draw_glow(ui, central_pos, [drop_size, drop_size], PulsarTheme::BLUE_GLOW, 0.8);

        ui.get_window_draw_list()
            .add_rect(central_pos, [central_pos[0] + drop_size, central_pos[1] + drop_size], PulsarTheme::BLUE_PRIMARY)
            .filled(true)
            .rounding(8.0)
            .build();

        // Side drop zones
        let side_thickness = 40.0;
        let side_length = 120.0;

        // Left
        let left_pos = [content_area[0] + 20.0, center_y - side_length / 2.0];
        VisualEffects::draw_glow(ui, left_pos, [side_thickness, side_length], PulsarTheme::BLUE_GLOW, 0.6);

        // Right
        let right_pos = [content_area[0] + content_area[2] - side_thickness - 20.0, center_y - side_length / 2.0];
        VisualEffects::draw_glow(ui, right_pos, [side_thickness, side_length], PulsarTheme::BLUE_GLOW, 0.6);

        // Top
        let top_pos = [center_x - side_length / 2.0, content_area[1] + 20.0];
        VisualEffects::draw_glow(ui, top_pos, [side_length, side_thickness], PulsarTheme::BLUE_GLOW, 0.6);

        // Bottom
        let bottom_pos = [center_x - side_length / 2.0, content_area[1] + content_area[3] - side_thickness - 20.0];
        VisualEffects::draw_glow(ui, bottom_pos, [side_length, side_thickness], PulsarTheme::BLUE_GLOW, 0.6);
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a saved layout configuration
#[derive(Debug, Clone)]
pub struct LayoutPreset {
    pub name: String,
    pub description: String,
    pub panels: Vec<PanelLayout>,
}

/// Individual panel configuration within a layout
#[derive(Debug, Clone)]
pub struct PanelLayout {
    pub panel_type: EditorPanelType,
    pub dock_area: DockArea,
    pub size_ratio: f32,
}

/// Available docking areas
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockArea {
    Left,
    Right,
    Top,
    Bottom,
    Center,
    Floating,
}

/// Grid system for precise panel positioning (inspired by CSS Grid)
pub struct GridSystem {
    pub columns: Vec<f32>,  // Column widths as ratios
    pub rows: Vec<f32>,     // Row heights as ratios
    pub gap: f32,           // Gap between grid cells
}

impl GridSystem {
    pub fn new(columns: Vec<f32>, rows: Vec<f32>) -> Self {
        Self {
            columns,
            rows,
            gap: 4.0,
        }
    }

    /// Calculate cell bounds for given grid position
    pub fn get_cell_bounds(&self, col: usize, row: usize, content_area: [f32; 4]) -> [f32; 4] {
        let total_col_ratio: f32 = self.columns.iter().sum();
        let total_row_ratio: f32 = self.rows.iter().sum();

        let cell_width = (content_area[2] - (self.columns.len() as f32 - 1.0) * self.gap) / total_col_ratio;
        let cell_height = (content_area[3] - (self.rows.len() as f32 - 1.0) * self.gap) / total_row_ratio;

        let x_offset: f32 = self.columns[..col].iter().sum::<f32>() * cell_width + col as f32 * self.gap;
        let y_offset: f32 = self.rows[..row].iter().sum::<f32>() * cell_height + row as f32 * self.gap;

        [
            content_area[0] + x_offset,
            content_area[1] + y_offset,
            self.columns[col] * cell_width,
            self.rows[row] * cell_height,
        ]
    }
}

/// Professional window management with smooth animations
pub struct WindowManager {
    pub windows: HashMap<String, WindowState>,
    pub animation_speed: f32,
}

#[derive(Debug, Clone)]
pub struct WindowState {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub target_position: [f32; 2],
    pub target_size: [f32; 2],
    pub is_visible: bool,
    pub is_focused: bool,
    pub alpha: f32,
    pub target_alpha: f32,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            animation_speed: 8.0,
        }
    }

    /// Update window animations
    pub fn update(&mut self, delta_time: f32) {
        for window in self.windows.values_mut() {
            let factor = self.animation_speed * delta_time;

            // Animate position
            window.position[0] += (window.target_position[0] - window.position[0]) * factor;
            window.position[1] += (window.target_position[1] - window.position[1]) * factor;

            // Animate size
            window.size[0] += (window.target_size[0] - window.size[0]) * factor;
            window.size[1] += (window.target_size[1] - window.size[1]) * factor;

            // Animate alpha
            window.alpha += (window.target_alpha - window.alpha) * factor;
        }
    }

    /// Animate window to new position/size
    pub fn animate_window(&mut self, window_id: &str, position: [f32; 2], size: [f32; 2]) {
        if let Some(window) = self.windows.get_mut(window_id) {
            window.target_position = position;
            window.target_size = size;
        }
    }
}