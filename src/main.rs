mod app;
mod player;

use app::App;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| {
            w.with_resizable(true).with_inner_size(
                dioxus::desktop::wry::application::dpi::LogicalSize::new(480.0, 120.0),
            )
        })
    });
}
