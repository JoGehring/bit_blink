mod ui;

use libadwaita::prelude::*;
use libadwaita::{Application};


fn main() {
    let application = Application::builder()
        .application_id("com.badge_magic_linux")
        .build();
    application.connect_startup(|_| ui::load_css());
    application.connect_activate(ui::build_ui);

    application.run();
}

