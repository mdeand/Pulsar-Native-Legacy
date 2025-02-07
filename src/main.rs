use gpui::{
    div, prelude::*, px, rgb, size, App, Application, 
    Bounds, Context, SharedString, TitlebarOptions, 
    Window, WindowBounds, WindowOptions
};

mod components; // We'll create this module for UI components
use components::{top_bar, menu_bar, tab_bar, main_content, status_bar, Tab};

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
            .child(top_bar("PULSAR ENGINE".into()))
            .child(menu_bar())
            .child(tab_bar(0, &vec!["Tab 1", "Tab 2", "Tab 3"]))
            .child(main_content(&Tab::LevelEditor))
            .child(status_bar(
                self.fps.clone(), 
                self.memory.clone(), 
                self.time.clone(), 
                self.branch.clone())
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    ..Default::default()
                }),
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