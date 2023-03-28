use libadwaita::Application;
use libadwaita::prelude::ApplicationExt;
use libadwaita::prelude::ApplicationExtManual;
use libadwaita::ApplicationWindow;
use libadwaita::prelude::GtkWindowExt;

pub fn main() {
    let application = Application::new(Some("com.example.gtk-rss-reader"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}