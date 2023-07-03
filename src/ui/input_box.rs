use std::boxed;

use libadwaita::glib::clone;
use libadwaita::gtk::{Box, CenterBox, Entry, Orientation};
use libadwaita::prelude::{BoxExt, ButtonExt, EditableExt};

use crate::ui::icon_grid;

/// **Combines the icon grid and the text entry**
///
/// * Creates a ```gtk::Box``` widget
/// * Calls the ```get_icon_grid``` method
/// * Create a ```Entry``` widget
/// * Combine everything together
/// * Set the Click event callback for every icon button to append the corresponding emoji to the entry widget value
///
/// # Returns
/// * A ```boxed::Box``` reference to the combined ```Box``` widget
/// * A static reference to the ```Entry``` widget for easy access
pub fn build_input_box() -> (boxed::Box<Box>, &'static Entry) {
    let input_box = Box::new(Orientation::Vertical, 5);
    let (icon_grid, icon_buttons) = icon_grid::get_icon_grid();
    let entry_box = Box::builder()
        .css_classes(["entry_box"])
        .orientation(Orientation::Vertical)
        .spacing(1)


        .build();
    let entry = Entry::builder()
        .can_focus(true)
        .focus_on_click(true)


        .placeholder_text("Type your text here...")
        .build();
    entry_box.append(&entry);
    input_box.append(&entry_box);
    input_box.append(&icon_grid);

    for button in icon_buttons {
        button.connect_clicked(clone!(@strong entry => move |button|{
            let mut entry_val = entry.text().as_str().to_string();
            entry_val.push_str(button.label().unwrap().as_str());
           entry.set_text(entry_val.as_str());
        }));
    }
    (boxed::Box::from(input_box), boxed::Box::leak(boxed::Box::from(entry)))
}