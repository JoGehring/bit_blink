use std::process::Command;

use libadwaita::ActionRow;
use libadwaita::gdk::cairo::LineCap::Butt;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, Button, CenterBox, Entry, Orientation, Widget};
use libadwaita::prelude::{ActionRowExt, BoxExt, ButtonExt, EditableExt};

pub fn build_bottom_box(entry: &Entry) -> (impl IsA<Widget>, Button) {
    let bottom_box = CenterBox::builder()
        .margin_top(5)
        .margin_end(5)
        .margin_bottom(50)
        .margin_start(5)
        .css_classes(["button"])
        .build();
    let button_box = Box::new(Orientation::Horizontal, 0);
    let save_button = Button::builder().margin_end(30).label("Save").build();
    save_button.connect_clicked(|_| { println!("Save!"); });
    let transfer_button = Button::builder().label("Transfer").build();
    let entry_copy = entry.clone();
    button_box.append(&save_button);
    button_box.append(&transfer_button);
    bottom_box.set_center_widget(Some(&button_box));
    (bottom_box, transfer_button)
}