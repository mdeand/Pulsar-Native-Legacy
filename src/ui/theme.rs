use imgui::*;

/// AMOLED Black theme colors matching the original Pulsar Engine design
pub struct PulsarTheme;

impl PulsarTheme {
    pub const PURE_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const DARK_PANEL: [f32; 4] = [0.02, 0.02, 0.02, 1.0];
    pub const DARKER_PANEL: [f32; 4] = [0.015, 0.015, 0.015, 1.0];
    pub const PANEL_BORDER: [f32; 4] = [0.12, 0.12, 0.12, 1.0];
    pub const SUBTLE_BORDER: [f32; 4] = [0.08, 0.08, 0.08, 1.0];

    // Blue accent colors - matching React app
    pub const BLUE_PRIMARY: [f32; 4] = [0.145, 0.388, 0.922, 1.0]; // #2563eb
    pub const BLUE_SECONDARY: [f32; 4] = [0.0, 0.2, 0.6, 1.0];
    pub const BLUE_HOVER: [f32; 4] = [0.2, 0.4, 0.9, 1.0];
    pub const BLUE_ACTIVE: [f32; 4] = [0.0, 0.15, 0.45, 1.0];
    pub const BLUE_GLOW: [f32; 4] = [0.145, 0.388, 0.922, 0.3];

    // Text colors
    pub const TEXT_PRIMARY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
    pub const TEXT_SECONDARY: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
    pub const TEXT_DISABLED: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
    pub const TEXT_MUTED: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

    // Tab colors
    pub const TAB_INACTIVE: [f32; 4] = [0.03, 0.03, 0.03, 1.0];
    pub const TAB_ACTIVE: [f32; 4] = [0.0, 0.1, 0.2, 1.0];
    pub const TAB_HOVER: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

    // Interactive elements
    pub const BUTTON_DEFAULT: [f32; 4] = [0.05, 0.05, 0.05, 1.0];
    pub const BUTTON_HOVER: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
    pub const BUTTON_ACTIVE: [f32; 4] = [0.15, 0.15, 0.15, 1.0];

    // Selection and highlight
    pub const SELECTION: [f32; 4] = [0.145, 0.388, 0.922, 0.4];
    pub const SELECTION_HOVER: [f32; 4] = [0.145, 0.388, 0.922, 0.6];
    pub const SELECTION_ACTIVE: [f32; 4] = [0.145, 0.388, 0.922, 0.8];

    // Scrollbar
    pub const SCROLLBAR_BG: [f32; 4] = [0.01, 0.01, 0.01, 0.5];
    pub const SCROLLBAR_GRAB: [f32; 4] = [0.2, 0.2, 0.2, 0.5];
    pub const SCROLLBAR_GRAB_HOVER: [f32; 4] = [0.25, 0.25, 0.25, 0.7];
    pub const SCROLLBAR_GRAB_ACTIVE: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

    /// Apply the complete Pulsar AMOLED theme to ImGui context
    pub fn apply_theme(ctx: &mut Context) {
        let style = ctx.style_mut();

        // Note: Docking not available in imgui 0.10.0

        // Window styling - Pure AMOLED black
        style.window_rounding = 0.0;
        style.window_border_size = 1.0;
        style.window_padding = [8.0, 8.0];
        style.window_min_size = [100.0, 50.0];

        // Frame styling (buttons, inputs, etc.)
        style.frame_rounding = 4.0;
        style.frame_border_size = 1.0;
        style.frame_padding = [12.0, 6.0];

        // Tab styling
        style.tab_rounding = 4.0;
        style.tab_border_size = 0.0;

        // Item spacing for professional look
        style.item_spacing = [8.0, 6.0];
        style.item_inner_spacing = [6.0, 4.0];
        style.indent_spacing = 20.0;

        // Scrollbar styling
        style.scrollbar_size = 16.0;
        style.scrollbar_rounding = 8.0;
        style.grab_rounding = 8.0;
        style.grab_min_size = 12.0;

        // Professional spacing
        style.columns_min_spacing = 8.0;
        style.popup_rounding = 6.0;
        style.popup_border_size = 1.0;
        style.child_rounding = 4.0;
        style.child_border_size = 1.0;

        // Apply AMOLED color scheme (simplified for imgui 0.10.0 compatibility)
        let colors = &mut style.colors;
        colors[StyleColor::Text as usize] = Self::TEXT_PRIMARY;
        colors[StyleColor::TextDisabled as usize] = Self::TEXT_DISABLED;
        colors[StyleColor::WindowBg as usize] = Self::PURE_BLACK;
        colors[StyleColor::ChildBg as usize] = Self::DARKER_PANEL;
        colors[StyleColor::PopupBg as usize] = Self::DARK_PANEL;
        colors[StyleColor::Border as usize] = Self::SUBTLE_BORDER;
        colors[StyleColor::BorderShadow as usize] = [0.0, 0.0, 0.0, 0.0];
        colors[StyleColor::FrameBg as usize] = Self::DARKER_PANEL;
        colors[StyleColor::FrameBgHovered as usize] = Self::DARK_PANEL;
        colors[StyleColor::FrameBgActive as usize] = Self::PANEL_BORDER;
        colors[StyleColor::TitleBg as usize] = Self::PURE_BLACK;
        colors[StyleColor::TitleBgActive as usize] = Self::DARKER_PANEL;
        colors[StyleColor::TitleBgCollapsed as usize] = Self::PURE_BLACK;
        colors[StyleColor::MenuBarBg as usize] = Self::DARKER_PANEL;
        colors[StyleColor::ScrollbarBg as usize] = Self::SCROLLBAR_BG;
        colors[StyleColor::ScrollbarGrab as usize] = Self::SCROLLBAR_GRAB;
        colors[StyleColor::ScrollbarGrabHovered as usize] = Self::SCROLLBAR_GRAB_HOVER;
        colors[StyleColor::ScrollbarGrabActive as usize] = Self::SCROLLBAR_GRAB_ACTIVE;
        colors[StyleColor::CheckMark as usize] = Self::BLUE_PRIMARY;
        colors[StyleColor::SliderGrab as usize] = Self::BLUE_PRIMARY;
        colors[StyleColor::SliderGrabActive as usize] = Self::BLUE_ACTIVE;
        colors[StyleColor::Button as usize] = Self::BUTTON_DEFAULT;
        colors[StyleColor::ButtonHovered as usize] = Self::BUTTON_HOVER;
        colors[StyleColor::ButtonActive as usize] = Self::BUTTON_ACTIVE;
        colors[StyleColor::Header as usize] = Self::DARKER_PANEL;
        colors[StyleColor::HeaderHovered as usize] = Self::DARK_PANEL;
        colors[StyleColor::HeaderActive as usize] = Self::PANEL_BORDER;
        colors[StyleColor::Separator as usize] = Self::SUBTLE_BORDER;
        colors[StyleColor::SeparatorHovered as usize] = Self::PANEL_BORDER;
        colors[StyleColor::SeparatorActive as usize] = Self::BLUE_PRIMARY;
        colors[StyleColor::ResizeGrip as usize] = Self::SUBTLE_BORDER;
        colors[StyleColor::ResizeGripHovered as usize] = Self::PANEL_BORDER;
        colors[StyleColor::ResizeGripActive as usize] = Self::BLUE_PRIMARY;
        colors[StyleColor::Tab as usize] = Self::TAB_INACTIVE;
        colors[StyleColor::TabHovered as usize] = Self::TAB_HOVER;
        colors[StyleColor::TabActive as usize] = Self::TAB_ACTIVE;
        colors[StyleColor::TabUnfocused as usize] = Self::TAB_INACTIVE;
        colors[StyleColor::TabUnfocusedActive as usize] = Self::TAB_ACTIVE;
        colors[StyleColor::PlotLines as usize] = Self::BLUE_PRIMARY;
        colors[StyleColor::PlotLinesHovered as usize] = Self::BLUE_HOVER;
        colors[StyleColor::PlotHistogram as usize] = Self::BLUE_PRIMARY;
        colors[StyleColor::PlotHistogramHovered as usize] = Self::BLUE_HOVER;
        colors[StyleColor::TextSelectedBg as usize] = Self::SELECTION;
    }

    /// Get style colors for buttons with variants
    pub fn button_style(variant: ButtonVariant) -> [f32; 4] {
        match variant {
            ButtonVariant::Primary => Self::BLUE_PRIMARY,
            ButtonVariant::Secondary => Self::BUTTON_DEFAULT,
            ButtonVariant::Danger => [0.7, 0.2, 0.2, 1.0],
            ButtonVariant::Success => [0.2, 0.7, 0.2, 1.0],
            ButtonVariant::Ghost => [0.0, 0.0, 0.0, 0.0],
        }
    }

    /// Get hover color for button variant
    pub fn button_hover_style(variant: ButtonVariant) -> [f32; 4] {
        match variant {
            ButtonVariant::Primary => Self::BLUE_HOVER,
            ButtonVariant::Secondary => Self::BUTTON_HOVER,
            ButtonVariant::Danger => [0.8, 0.3, 0.3, 1.0],
            ButtonVariant::Success => [0.3, 0.8, 0.3, 1.0],
            ButtonVariant::Ghost => [0.1, 0.1, 0.1, 0.5],
        }
    }

    /// Get active color for button variant
    pub fn button_active_style(variant: ButtonVariant) -> [f32; 4] {
        match variant {
            ButtonVariant::Primary => Self::BLUE_ACTIVE,
            ButtonVariant::Secondary => Self::BUTTON_ACTIVE,
            ButtonVariant::Danger => [0.6, 0.1, 0.1, 1.0],
            ButtonVariant::Success => [0.1, 0.6, 0.1, 1.0],
            ButtonVariant::Ghost => [0.15, 0.15, 0.15, 0.7],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Success,
    Ghost,
}

/// Helper for drawing gradients and glows (simulated with rectangles)
pub struct VisualEffects;

impl VisualEffects {
    /// Draw a subtle glow effect around a rectangle
    pub fn draw_glow(ui: &Ui, pos: [f32; 2], size: [f32; 2], color: [f32; 4], intensity: f32) {
        let draw_list = ui.get_window_draw_list();
        let glow_size = 4.0 * intensity;

        // Multiple layers for smooth glow effect
        for i in 1..4 {
            let layer_alpha = color[3] * (0.3 / i as f32) * intensity;
            let layer_color = [color[0], color[1], color[2], layer_alpha];
            let expand = glow_size * i as f32;

            draw_list
                .add_rect(
                    [pos[0] - expand, pos[1] - expand],
                    [pos[0] + size[0] + expand, pos[1] + size[1] + expand],
                    layer_color,
                )
                .filled(true)
                .rounding(2.0)
                .build();
        }
    }

    /// Draw a radial gradient effect (simulated)
    pub fn draw_radial_gradient(
        ui: &Ui,
        center: [f32; 2],
        radius: f32,
        inner_color: [f32; 4],
        outer_color: [f32; 4],
    ) {
        let draw_list = ui.get_window_draw_list();
        let steps = 20;

        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let current_radius = radius * t;
            let alpha = inner_color[3] * (1.0 - t) + outer_color[3] * t;
            let color = [
                inner_color[0] * (1.0 - t) + outer_color[0] * t,
                inner_color[1] * (1.0 - t) + outer_color[1] * t,
                inner_color[2] * (1.0 - t) + outer_color[2] * t,
                alpha,
            ];

            draw_list
                .add_circle(center, current_radius, color)
                .filled(true)
                .num_segments(32)
                .build();
        }
    }
}