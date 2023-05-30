extern crate core;
use libadwaita::Application;
use libadwaita::gtk::Box;
use libadwaita::prelude::*;

use crate::ui::window;

mod ui;
mod bluetooth;
mod storage;

#[tokio::main]
async fn main() {
    let application = Application::builder()
        .application_id("com.badge_magic_linux")
        .build();
    application.connect_startup(|_| ui::load_css());


    application.connect_activate(show_window);
    application.run();
}

fn show_window(application: &Application) {
    let content = ui::build_ui();
    let app_window = window::create_window(&application, &content);
    app_window.show();
}