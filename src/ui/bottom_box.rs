use libadwaita::ActionRow;
use libadwaita::gdk::cairo::LineCap::Butt;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Orientation, Box, Button, CenterBox, Widget};
use libadwaita::prelude::{ActionRowExt, BoxExt, ButtonExt};

pub fn build_bottom_box() -> impl IsA<Widget> {
    let bottom_box = CenterBox::builder()
        .margin_top(5)
        .margin_end(5)
        .margin_bottom(50)
        .margin_start(5)
        .build();
    let button_box = Box::new(Orientation::Horizontal, 0);
    let save_button = Button::builder().margin_end(30).label("Save").build();
    save_button.connect_clicked(|_|{ println!("Save!");});
    let transfer_button = Button::builder().label("Transfer").build();
    transfer_button.connect_clicked(|_|{ println!("Transfer!");});
    button_box.append(&save_button);
    button_box.append(&transfer_button);
    bottom_box.set_center_widget(Some(&button_box));
    bottom_box
}