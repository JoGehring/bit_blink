use libadwaita::{Application, ApplicationWindow};

pub fn create_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("BitBlink")
        .default_width(720)
        .default_height(1280)
        .resizable(true)
        .build()
}