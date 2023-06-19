use std::boxed;

use libadwaita::gtk::{Box, Orientation, Scale};
use libadwaita::prelude::{BoxExt, RangeExt};

pub fn build_speed_page() -> (boxed::Box<Box>, boxed::Box<Scale>) {
    let speed_page = Box::builder()
        .css_classes(["speed_page"])
        .build();
    let scale = Scale::builder()
        .orientation(Orientation::Horizontal)
        .vexpand(true)
        .name("Speed".to_string())
        .draw_value(true)
        .hexpand(true)
        .round_digits(0)
        .digits(0)
        .build();
    scale.set_range(1.0, 8.0);

    speed_page.append(&scale);
    (boxed::Box::from(speed_page), boxed::Box::from( scale))
}