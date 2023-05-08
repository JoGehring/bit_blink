use libadwaita::{Application, ApplicationWindow};
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box};

pub fn create_window(app: &Application, content: &Box) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Badge Magic")
        .default_width(500)
        .default_height(880)
        // add content to window
        .content(content)
        .build()
}