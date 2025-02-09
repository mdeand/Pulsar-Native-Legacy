use std::{thread::sleep, time::Duration};

use gpui::{*, App as Application};
// use tokio::time::sleep;

mod app;
mod components;

#[tokio::main]
async fn main() {
    let app = Application::new();
    
    app.background_executor().spawn((async || loop {
        sleep(Duration::from_secs(1));
        println!("testing");
    })()).detach();


    app.run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    title: Some("Pulsar Engine".into()),
                    ..Default::default()
                }),

                ..Default::default()
            },
            |cx| {
                app::App::new(cx)
            },
        )
        .unwrap();
    })
}