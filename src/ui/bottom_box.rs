
use libadwaita::gtk::{Box, Button, CenterBox, Image, Label, Orientation};
use libadwaita::prelude::{BoxExt, ButtonExt};
use std::boxed;

pub fn build_bottom_box() -> (boxed::Box<CenterBox>, boxed::Box<Button>, boxed::Box<Button>) {
    let bottom_box = CenterBox::builder()
        .margin_top(5)
        .margin_end(5)
        .margin_bottom(5)
        .margin_start(5)
        .css_classes(["bottom_box"])
        .build();
    let button_box = Box::new(Orientation::Horizontal, 0);
    let save_button_label_box = Box::new(Orientation::Horizontal, 5);
    save_button_label_box.prepend(&Image::from_icon_name("document-save"));
    let save_button = Button::builder().margin_end(30).build();
    save_button_label_box.append(&Label::new(Some("Save")));
    save_button.set_child(Some(&save_button_label_box));
    let transfer_button_label_box = Box::new(Orientation::Horizontal, 5);
    transfer_button_label_box.prepend(&Image::from_icon_name("go-up"));
    transfer_button_label_box.append(&Label::new(Some("Transfer")));

    let transfer_button = Button::builder().build();
    transfer_button.set_child(Some(&transfer_button_label_box));
    button_box.append(&save_button);
    button_box.append(&transfer_button);
    bottom_box.set_center_widget(Some(&button_box));
    (boxed::Box::from(bottom_box), boxed::Box::from(save_button), boxed::Box::from(transfer_button))
}