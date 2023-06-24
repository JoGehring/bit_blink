extern crate core;

use std::boxed;

use libadwaita::{Application, ApplicationWindow, gtk};
use libadwaita::gtk::{Box, Orientation};
use libadwaita::prelude::*;

use crate::ui::window;

mod ui;
mod bluetooth;
mod storage;

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
    let mut content = boxed::Box::from(Box::new(Orientation::Vertical, 0));
    let APP_WINDOW = boxed::Box::from(window::create_window(&application));
    ui::build_ui(std::boxed::Box::<libadwaita::ApplicationWindow>::leak(APP_WINDOW), &content);
}