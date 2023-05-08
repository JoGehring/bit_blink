use libadwaita::ActionRow;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, DropDown, Label, Widget};
use libadwaita::prelude::{ActionRowExt, BoxExt, WidgetExt};

pub fn build_animations_page() -> (impl IsA<Widget>, DropDown) {
    let animations_page = Box::builder()
        .margin_top(250)
        .margin_end(5)
        .margin_bottom(250)
        .margin_start(5)
        .build();
    let animation_label = Label::builder().label("Animation").build();
    let drop_down = DropDown::from_strings(["Scroll Left", "Scroll Right", "Scroll Up", "Scroll Down", "Still Centered", "Animation", "Drop Down", "Curtain", "Laser"].as_ref());
    drop_down.set_hexpand(true);
    animations_page.append(&animation_label);
    animations_page.append(&drop_down);
    (animations_page, drop_down)
}