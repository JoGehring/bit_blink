use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, Orientation, Scale, Widget};
use libadwaita::prelude::{BoxExt, RangeExt};

pub fn build_speed_page() -> impl IsA<Widget> {
    let speed_page = Box::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .build();
    let scale = Scale::builder()
        .orientation(Orientation::Horizontal)
        .vexpand(true)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .name("Speed".to_string())
        .draw_value(true)
        .hexpand(true)
        .round_digits(0)
        .digits(0)
        .build();
    scale.set_range(0.0, 100.0);

    speed_page.append(&scale);
    speed_page
}