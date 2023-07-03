use std::boxed;

use libadwaita::gtk::{Box, Orientation, Scale};
use libadwaita::prelude::{BoxExt, RangeExt};

/// **Builds the widget for setting the speed of the message**
///
/// * Creates a ```gtk::Box``` widget
/// * Create a ```Scale``` widget
/// * Combine the widgets together
///
/// # Returns
/// * A ```boxed::Box``` reference to the combined ```Box``` widget
/// * A ```boxed::Box``` reference to the ```Scale``` widget for easy access
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
    (boxed::Box::from(speed_page), boxed::Box::from(scale))
}