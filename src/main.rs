use app::App;
use gpui::{*, App as Application};

mod app;

fn main() {
    Application::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| {
                app::App::new(cx)
            },
        )
        .unwrap();
    });
}