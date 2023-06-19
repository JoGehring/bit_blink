

use libadwaita::glib::{clone};
use libadwaita::gtk::{Box, CenterBox, Entry, Orientation};

use libadwaita::prelude::{BoxExt, ButtonExt, EditableExt};
use std::boxed;
use crate::ui::icon_grid;

pub fn build_input_box() -> (boxed::Box<Box>, boxed::Box<Entry>) {
    let input_box = Box::new(Orientation::Vertical, 5);
    let (icon_grid, icon_buttons) = icon_grid::get_icon_grid();
    let entry_box = CenterBox::builder().css_classes(["entry_box"]).build();
    let entry = Entry::builder().can_focus(true).focus_on_click(true).hexpand(true).vexpand(true).placeholder_text("Type your text here...").build();
    entry_box.set_center_widget(Some(&entry));
    input_box.append(&entry_box);
    input_box.append(&icon_grid);

    for button in icon_buttons {
        button.connect_clicked(clone!(@strong entry => move |button|{
            let mut entry_val = entry.text().as_str().to_string();
            entry_val.push_str(button.label().unwrap().as_str());
           entry.set_text(entry_val.as_str());
        }));
    }
    (boxed::Box::from(input_box), boxed::Box::from(entry))
}