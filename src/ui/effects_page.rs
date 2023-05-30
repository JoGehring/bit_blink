use libadwaita::glib::IsA;
use libadwaita::gtk::{CenterBox, ToggleButton, Widget};
use std::boxed;
pub fn build_effects_page() -> (Box<CenterBox>, Box<ToggleButton>, Box<ToggleButton>, Box<ToggleButton>) {
    let effects_page = CenterBox::builder()
        .css_classes(["bottom_box", "animations"])
        .build();

    let flash_button = ToggleButton::builder().label("Flash").build();
    let marquee_button = ToggleButton::builder().label("Marquee").build();
    let invert_button = ToggleButton::builder().label("Invert").build();
    effects_page.set_start_widget(Some(&flash_button));
    effects_page.set_center_widget(Some(&marquee_button));
    effects_page.set_end_widget(Some(&invert_button));
    (Box::from(effects_page), Box::from(flash_button), Box::from(marquee_button), Box::from(invert_button))
}