extern crate core;

use std::boxed;

use libadwaita::{Application, ApplicationWindow, gtk};
use libadwaita::prelude::*;

use crate::ui::window;

mod bluetooth;
mod storage;
mod ui;


/// **Main entry point for the application**
///
/// # Workflow
///
/// - creates a Application object
/// - connects the callback for setting the CSS properties
/// - connects the callback for initializing and updating the ApplicationWindow
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


/// **Creates a new ApplicationWindow and passes it to build_ui**
///
/// # Arguments
///
/// * `application` - the application object created in the ```main```-function
///
/// # Workflow
///
/// - creates a ApplicationWindow object
/// - leaks a static reference to the window to build_ui, as its supposed to 'live' as long as the program is running
fn show_window(application: &Application) {
    let app_window = boxed::Box::from(window::create_window(&application));
    ui::build_ui(
        boxed::Box::<ApplicationWindow>::leak(app_window),
        None);
}
