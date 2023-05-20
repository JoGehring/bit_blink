use std::fmt::Display;

use libadwaita::glib::{clone, GString, IsA, MainContext};
use libadwaita::gtk::{Box, CenterBox, Entry, Orientation};
use libadwaita::gtk::Widget;
use libadwaita::prelude::{BoxExt, ButtonExt, EditableExt};

use crate::ui::icon_grid;

pub fn build_input_box() -> (Box, Entry) {
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
    (input_box, entry)
}