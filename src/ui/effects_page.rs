
use libadwaita::gtk::{Box, ToggleButton, Orientation, CenterBox};
use libadwaita::prelude::{BoxExt, ButtonExt};

pub fn build_effects_page() -> (std::boxed::Box<CenterBox>, std::boxed::Box<ToggleButton>, std::boxed::Box<ToggleButton>, std::boxed::Box<ToggleButton>) {
    let container  = Box::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(["animations"])
        .spacing(10)
        .build();

    let flash_button = ToggleButton::builder().label("Flash").css_classes(["effect_button"]).vexpand_set(false).build();
    let marquee_button = ToggleButton::builder().label("Marquee").css_classes(["effect_button"]).vexpand_set(false).build();
    let invert_button = ToggleButton::builder().label("Invert").css_classes(["effect_button"]).vexpand_set(false).build();
    container.append(&flash_button);
    container.append(&marquee_button);
    container.append(&invert_button);
    let effects_page = CenterBox::builder().css_classes(["effects_page"]).build();
    effects_page.set_center_widget(Some(&container));
    (std::boxed::Box::from(effects_page), std::boxed::Box::from(flash_button), std::boxed::Box::from(marquee_button), std::boxed::Box::from(invert_button))
}