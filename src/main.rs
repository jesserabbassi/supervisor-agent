mod alerts;
mod app;
mod auth;
mod customers;
mod dashboard;
mod infrastructure;
mod monitoring;
mod reports;
mod reservations;
mod sessions;
mod settings;
mod shared;
mod stations;
mod wallet;

use app::SupervisorApp;
use gpui::{px, size, App, AppContext, Bounds, WindowBounds, WindowOptions};
use gpui_platform::application;
use std::borrow::Cow;

fn main() {
    application().run(|cx: &mut App| {
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(
                    include_bytes!("../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf")
                        .as_slice(),
                ),
                Cow::Borrowed(
                    include_bytes!("../assets/fonts/ibm-plex-sans/IBMPlexSans-SemiBold.ttf")
                        .as_slice(),
                ),
            ])
            .expect("load application fonts");
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1500.), px(950.)),
                cx,
            ))),
            ..WindowOptions::default()
        };
        cx.open_window(options, |_, cx| cx.new(|_| SupervisorApp::new()))
            .expect("open supervisor window");
        cx.activate(true);
    });
}
