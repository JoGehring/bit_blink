use libadwaita::ActionRow;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, CenterBox, ToggleButton, Widget};
use libadwaita::prelude::ActionRowExt;

pub fn build_effects_page() -> (impl IsA<Widget>, ToggleButton, ToggleButton, ToggleButton) {
    let effects_page = CenterBox::builder()
        .margin_top(280)
        .margin_end(30)
        .margin_bottom(280)
        .margin_start(30)
        .css_classes(["button"])
        .build();

    let flash_button = ToggleButton::builder().label("Flash").build();
    let marquee_button = ToggleButton::builder().label("Marquee").build();
    let invert_button = ToggleButton::builder().label("Invert").build();
    effects_page.set_start_widget(Some(&flash_button));
    effects_page.set_center_widget(Some(&marquee_button));
    effects_page.set_end_widget(Some(&invert_button));
    (effects_page, flash_button, marquee_button, invert_button)
}