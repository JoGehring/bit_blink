use libadwaita::{Application, ApplicationWindow};

pub fn create_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("BitBlink")
        .resizable(true)
        .build()
}