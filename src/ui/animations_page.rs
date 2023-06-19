use std::boxed;

use libadwaita::gtk::{Box, DropDown};
use libadwaita::prelude::{BoxExt, WidgetExt};

pub fn build_animations_page() -> (boxed::Box<Box>, boxed::Box<DropDown>) {
    let animations_page = Box::builder()
        .css_classes(["entry_box", "animations"])
        .build();
    let drop_down = DropDown::from_strings(["Scroll Left", "Scroll Right", "Scroll Up", "Scroll Down", "Still Centered", "Animation", "Drop Down", "Curtain", "Laser"].as_ref());
    drop_down.set_hexpand(true);
    let drop_down_list = drop_down.last_child().unwrap();
    drop_down_list.add_css_class("drop_down_list");
    animations_page.append(&drop_down);
    (boxed::Box::from(animations_page), boxed::Box::from(drop_down))
}