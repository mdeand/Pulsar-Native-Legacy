use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

struct GameEngine {
    title: SharedString,
    branch: SharedString,
    fps: SharedString,
    memory: SharedString,
    time: SharedString,
}

impl Render for GameEngine {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x000000)) // Pure black background
            .child(
                // Top bar with title and controls
                div()
                    .flex()
                    .h_10()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .bg(rgb(0x0A0A0A))
                    .child(
                        // Title and icon
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_color(rgb(0x2F80ED))
                            .child("◆")
                            .child(
                                div()
                                    .text_color(rgb(0x2F80ED))
                                    .text_sm()
                                    .child("PULSAR ENGINE")
                            )
                    )
                    .child(
                        // Window controls
                        div()
                            .flex()
                            .gap_4()
                            .child(div().text_lg().text_color(rgb(0x666666)).child("−"))
                            .child(div().text_lg().text_color(rgb(0x666666)).child("□"))
                            .child(div().text_lg().text_color(rgb(0x666666)).child("×"))
                    )
            )
            .child(
                // Menu bar
                div()
                    .flex()
                    .h_8()
                    .items_center()
                    .px_4()
                    .gap_6()
                    .bg(rgb(0x0A0A0A))
                    .text_color(rgb(0xCCCCCC))
                    .text_sm()
                    .child("File")
                    .child("Edit")
                    .child("View")
                    .child("Project")
                    .child("Build")
                    .child("Tools")
                    .child("Settings")
                    .child("Help")
                    .child(
                        div()
                            .flex_grow()
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child("⚙")
                                    .child("Settings")
                            )
                    )
            )
            .child(
                // Tab bar
                div()
                    .flex()
                    .h_8()
                    .items_center()
                    .px_2()
                    .gap_1()
                    .bg(rgb(0x0A0A0A))
                    .text_color(rgb(0x666666))
                    .text_sm()
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .text_color(rgb(0x666666))
                            .child("Level Editor ×")
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x1A1A1A))
                            .text_color(rgb(0xFFFFFF))
                            .child("Script Editor ×")
                    )
                    .child(
                        div()
                            .flex_grow()
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .px_2()
                                    .child("+ ")
                            )
                    )
            )
            .child(
                // Main content area
                div()
                    .flex()
                    .flex_grow()
                    .child(
                        // Explorer sidebar
                        div()
                            .w_64()
                            .bg(rgb(0x0A0A0A))
                            .border_color(rgb(0x222222))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px_4()
                                    .py_2()
                                    .text_color(rgb(0xCCCCCC))
                                    .text_sm()
                                    .child("EXPLORER")
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child("+")
                                            .child("□")
                                            .child("↻")
                                    )
                            )
                    )
                    .child(
                        // Main editor area
                        div()
                            .flex_grow()
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_4()
                                    .text_color(rgb(0x666666))
                                    .child("↳")
                                    .child("Welcome to Quasar")
                                    .child("Open a file to start editing")
                                    .child(
                                        div()
                                            .flex()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .px_4()
                                                    .py_2()
                                                    .bg(rgb(0x2F80ED))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .rounded_sm()
                                                    .child("Open Folder")
                                            )
                                            .child(
                                                div()
                                                    .px_4()
                                                    .py_2()
                                                    .bg(rgb(0x1A1A1A))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .rounded_sm()
                                                    .child("New File")
                                            )
                                    )
                            )
                    )
            )
            .child(
                // Status bar
                div()
                    .flex()
                    .h_6()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .bg(rgb(0x0A0A0A))
                    .text_color(rgb(0xCCCCCC))
                    .text_xs()
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .items_center()
                            .child("⚠ Rust Analyzer: 2 issues")
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .text_color(rgb(0xFFA500))
                                    .child("⌥")
                                    .child("feature/physics-update")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .items_center()
                            .child(format!("FPS: {}", self.fps))
                            .child(format!("{} MB", self.memory))
                            .child(self.time.to_string())
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| GameEngine {
                    title: "PULSAR ENGINE".into(),
                    branch: "feature/physics-update".into(),
                    fps: "3001".into(),
                    memory: "548".into(),
                    time: "11:40:19 PM".into(),
                })
            },
        )
        .unwrap();
    });
}