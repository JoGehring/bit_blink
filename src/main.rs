extern crate core;

use std::boxed;

use libadwaita::{Application, ApplicationWindow, gtk};
use libadwaita::prelude::*;

use crate::ui::window;

mod bluetooth;
mod storage;
mod ui;

#[tokio::main]
async fn main() {
    gtk::init().expect("Failed to initialize");
    let application = Application::builder()
        .application_id("com.bit_blink")
        .build();
    application.connect_startup(|_| ui::load_css());
    application.connect_activate(show_window);
    application.run();
}

fn show_window(application: &Application) {
    let app_window = boxed::Box::from(window::create_window(&application));
    ui::build_ui(
        boxed::Box::<ApplicationWindow>::leak(app_window),
        None);
}
