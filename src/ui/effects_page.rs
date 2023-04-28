use libadwaita::ActionRow;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, CenterBox, ToggleButton, Widget};
use libadwaita::prelude::ActionRowExt;

pub fn build_effects_page() -> impl IsA<Widget> {
    let effects_page = CenterBox::builder()
        .margin_top(5)
        .margin_end(5)
        .margin_bottom(5)
        .margin_start(5)
        .css_classes(["test"])
        .build();

    let flash_button = ToggleButton::builder().label("Flash").build();
    let marquee_button = ToggleButton::builder().label("Marquee").build();
    let invert_button = ToggleButton::builder().label("Invert").build();
    effects_page.set_start_widget(Some(&flash_button));
    effects_page.set_center_widget(Some(&marquee_button));
    effects_page.set_end_widget(Some(&invert_button));
    effects_page
}