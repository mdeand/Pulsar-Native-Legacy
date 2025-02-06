use gpui::{
    div, prelude::*, px, rgb, size, App, Application, 
    Bounds, Context, SharedString, TitlebarOptions, 
    Window, WindowBounds, WindowOptions
};

mod components; // We'll create this module for UI components
use components::{TopBar, MenuBar, TabBar, MainContent, StatusBar};

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
            .child(TopBar::new(self.title.clone()))
            .child(MenuBar::new())
            .child(TabBar::new())
            .child(MainContent::new())
            .child(StatusBar::new(
                self.fps.clone(), 
                self.memory.clone(), 
                self.time.clone(), 
                self.branch.clone()
            ))
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