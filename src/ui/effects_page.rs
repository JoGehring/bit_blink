
use libadwaita::gtk::{CenterBox, ToggleButton};

pub fn build_effects_page() -> (Box<CenterBox>, Box<ToggleButton>, Box<ToggleButton>, Box<ToggleButton>) {
    let effects_page = CenterBox::builder()
        .css_classes(["animations"])
        .build();

    let flash_button = ToggleButton::builder().label("Flash").css_classes(["effect_button"]).build();
    let marquee_button = ToggleButton::builder().label("Marquee").css_classes(["effect_button"]).build();
    let invert_button = ToggleButton::builder().label("Invert").css_classes(["effect_button"]).build();
    effects_page.set_start_widget(Some(&flash_button));
    effects_page.set_center_widget(Some(&marquee_button));
    effects_page.set_end_widget(Some(&invert_button));
    (Box::from(effects_page), Box::from(flash_button), Box::from(marquee_button), Box::from(invert_button))
}