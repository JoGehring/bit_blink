use libadwaita::{Application, ApplicationWindow};
use libadwaita::glib::IsA;
use libadwaita::gtk::{Widget};

pub fn create_window(app: &Application, content: &impl IsA<Widget>) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Badge Magic")
        .default_width(500)
        .default_height(880)
        // add content to window
        .content(content)
        .build()
}