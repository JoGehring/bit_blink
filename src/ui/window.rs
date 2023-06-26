use libadwaita::{Application, ApplicationWindow};
use libadwaita::gtk::{Box};

pub fn create_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("BitBlink")
        .default_width(720)
        .default_height(1280)
        // add content to window
        // .content(content)
        // .maximized(true)
        .resizable(true)
        .build()
}