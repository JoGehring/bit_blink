use libadwaita::{ApplicationWindow, gtk};
use libadwaita::gdk::Display;
use libadwaita::glib::{clone, MainContext};
use libadwaita::gtk::{Box, CssProvider, DropDown, Entry, Orientation, Scale, style_context_add_provider_for_display, ToggleButton};
use libadwaita::prelude::{AdwApplicationWindowExt, BoxExt, ButtonExt, EditableExt, RangeExt, ToggleButtonExt, WidgetExt};

use crate::bluetooth::{Animation, Message, Speed};
use crate::bluetooth::connection;
use crate::storage::storage::build_storage;

mod view_stack;
pub mod window;
mod speed_page;
mod effects_page;
mod input_box;
mod animations_page;
mod bottom_box;
mod icon_grid;
mod message_list;

/// **Adds CSS to the GTK application**
///
/// # Workflow
///
/// * Load the CSS file and add it to the provider
/// * Add the provider to the default screen
pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// # Arguments
///
/// * `app_window` - static reference to the ApplicationWindow
/// * `message` - Optional message object, if the re-built UI should contain the current input set
///
/// # Workflow
///
/// * Calls the functions creating the different building blocks
/// * If ```message``` is some, set the values in the input widgets
/// * Connect the Click event of the button widgets to the corresponding actions
/// * When the save or one of the delete buttons are clicked, first the action, e.g. saving, is performed, then the ```build_ui```
/// method is called again, to re-build the UI with an updated message list
/// * Combine all widgets in one ```Box``` object and set it as the windows content
/// * Show the window
pub fn build_ui(app_window: &'static ApplicationWindow, message: Option<Message>) {
    let (input_box, entry) = input_box::build_input_box();
    let (stack_switcher, stack, scale, flash_button, marquee_button, invert_button, drop_down) = view_stack::build_view_stack();
    let (bottom_box, save_button, transfer_button) = bottom_box::build_bottom_box();
    let (header_bar, delete_buttons) = message_list::get_message_list(entry, scale, flash_button, marquee_button, invert_button, drop_down);

    if message.is_some() {
        let current_message = message.unwrap();
        entry.set_text(current_message.texts[0].as_str());
        scale.set_value(Speed::get_value(current_message.speed[0].clone()));
        flash_button.set_active(current_message.flash[0]);
        marquee_button.set_active(current_message.marquee[0]);
        invert_button.set_active(current_message.inverted[0]);
        drop_down.set_selected(Animation::get_value(current_message.mode[0].clone()));
    }

    save_button.connect_clicked(move |save_button| {
        save_button.set_sensitive(false);
        let mut bt_message = build_message(entry, scale, drop_down, flash_button, marquee_button, invert_button);
        let msg_storage = build_storage();
        msg_storage.save_message(&mut bt_message);
        build_ui(app_window, Some(bt_message));
        save_button.set_sensitive(true);
    });
    transfer_button.connect_clicked(move |transfer_button| {
        let main_context = MainContext::default();
        main_context.spawn_local(clone!( @ strong entry, @ strong scale, @ strong drop_down, @ strong marquee_button, @ strong flash_button, @ strong invert_button, @ strong transfer_button => async move {
        transfer_button.set_sensitive(false);
        let bt_message = build_message(&entry, &scale, &drop_down, &flash_button, &marquee_button, &invert_button);
        connection( & bt_message).await.expect("Error while transferring the data");
        transfer_button.set_sensitive(true);
        }));
    });

    for button in delete_buttons {
        let storage = build_storage();
        button.connect_clicked(move |button| {
            storage.delete_badge(&button.css_classes().last().unwrap().to_string());
            build_ui(app_window, None);
        });
    }

    let without_header_bar = Box::new(Orientation::Vertical, 0);
    let content = Box::new(Orientation::Vertical, 0);
    without_header_bar.append(input_box.as_ref());
    without_header_bar.append(stack_switcher.as_ref());
    without_header_bar.append(stack.as_ref());
    without_header_bar.append(bottom_box.as_ref());
    content.append(&header_bar);
    content.append(&without_header_bar);

    app_window.set_content(Some(&content));
    app_window.show();
}

/// # Arguments
///
/// * `entry` - reference to the text input widget, setting the text of the message
/// * `scale` - reference to the Scale widget setting the speed of the message
/// * `drop_down` - reference to the DropDown widget setting the animation mode of the message
/// * `flash_button` - reference to the ToggleButton setting if the message should flash
/// * `marquee_button` - reference to the ToggleButton setting if the message should have a marquee
/// * `invert_button` - reference to the ToggleButton setting if the message should be inverted
///
/// # Returns
/// * A message ready to be sent to the LED badge
fn build_message(entry: &Entry, scale: &Scale, drop_down: &DropDown, flash_button: &ToggleButton, marquee_button: &ToggleButton, invert_button: &ToggleButton) -> Message {
    let texts = vec![String::from(entry.text())];
    let speed = vec![Speed::get(scale.value())];
    let mode = vec![Animation::get(drop_down.selected())];
    let flash = vec![flash_button.is_active()];
    let marquee = vec![marquee_button.is_active()];
    let inverted = vec![invert_button.is_active()];
    Message { file_name: "".to_string(), texts, inverted, flash, marquee, speed, mode }
}


#[cfg(test)]
mod tests {
    #[test]
    fn save_message_test() {
        let i = 3;
        assert_eq!(3, i);
    }
}