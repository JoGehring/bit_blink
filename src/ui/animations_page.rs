use libadwaita::ActionRow;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, Widget};
use libadwaita::prelude::ActionRowExt;

pub fn build_animations_page() -> impl IsA<Widget> {
    let animations_page = Box::builder()
        .margin_top(5)
        .margin_end(5)
        .margin_bottom(5)
        .margin_start(5)
        .build();

    animations_page
}