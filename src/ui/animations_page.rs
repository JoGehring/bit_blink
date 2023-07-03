use std::boxed;

use libadwaita::gtk::{Box, DropDown, Popover, PositionType};
use libadwaita::prelude::{BoxExt, WidgetExt, PopoverExt, Cast};

/// **Builds the widget for setting the animation mode**
///
/// * Creates a ```gtk::Box``` widget
/// * Creates a ```DropDown``` widget
/// * Change the position type of the ```Popover``` child, so that it opens up to the top
///
/// # Returns
/// * A ```boxed::Box``` reference to the combined ```gtk::Box``` widget
/// * A ```boxed::Box``` reference to the ```DropDown``` widget for easy access
pub fn build_animations_page() -> (boxed::Box<Box>, boxed::Box<DropDown>) {
    let animations_page = Box::builder()
        .css_classes(["effects_page"])
        .build();
    let drop_down = DropDown::from_strings(["Scroll Left", "Scroll Right", "Scroll Up", "Scroll Down", "Still Centered", "Animation", "Drop Down", "Curtain", "Laser"].as_ref());
    drop_down.set_hexpand(true);
    drop_down.last_child().unwrap().downcast::<Popover>().unwrap().set_position(PositionType::Top);
    animations_page.append(&drop_down);
    (boxed::Box::from(animations_page), boxed::Box::from(drop_down))
}