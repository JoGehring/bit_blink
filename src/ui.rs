use std::boxed;
use libadwaita::{gtk, HeaderBar};
use libadwaita::gdk::Display;
use libadwaita::glib::{clone, MainContext};
use libadwaita::gtk::{Box, CssProvider, DropDown, EmojiChooser, Entry, Label, MenuButton, Orientation, Popover, PositionType, Scale, StyleContext, ToggleButton};
use libadwaita::prelude::{BoxExt, ButtonExt, EditableExt, ObjectExt, PopoverExt, RangeExt, ToggleButtonExt, WidgetExt};

use crate::bluetooth::{Animation, Message, Speed};
use crate::bluetooth::connection;
use crate::storage::storage;

mod view_stack;
pub mod window;
mod speed_page;
mod effects_page;
mod input_box;
mod animations_page;
mod bottom_box;
mod header_bar;
mod message_list;
mod icon_grid;


pub fn build_ui() -> boxed::Box<Box> {
    let (input_box, entry) = input_box::build_input_box();
    let (stack_switcher, stack, scale, flash_button, marquee_button, invert_button, drop_down) = view_stack::build_view_stack();
    let (bottom_box, save_button, transfer_button) = bottom_box::build_bottom_box();
    let content = Box::new(Orientation::Vertical, 0);
    let header_bar = header_bar::build_header_bar();
    content.append(header_bar.as_ref());
    content.append(input_box.as_ref());
    content.append(stack_switcher.as_ref());
    content.append(stack.as_ref());
    content.append(bottom_box.as_ref());
    let flash_clone = flash_button.clone();
    let scale_clone = scale.clone();
    let invert_clone = invert_button.clone();
    let marquee_clone = marquee_button.clone();
    let drop_down_clone = drop_down.clone();
    let entry_clone = entry.clone();
    save_button.connect_clicked(move |save_button| {
        save_button.set_sensitive(false);
        let bt_message = build_message(&entry_clone, &scale_clone, &drop_down_clone, &flash_clone, &marquee_clone, &invert_clone);
        let msg_storage = storage::build_storage();
        msg_storage.save_message(&bt_message);
        save_button.set_sensitive(true);
    });
    transfer_button.connect_clicked(move |transfer_button| {
        let main_context = MainContext::default();
        main_context.spawn_local(clone!( @ strong entry, @ strong scale, @ strong drop_down, @ strong marquee_button, @ strong flash_button, @ strong invert_button, @ strong transfer_button => async move {
        transfer_button.set_sensitive(false);
        let bt_message = build_message( & entry, & scale, & drop_down, & flash_button, & marquee_button, & invert_button);
        connection( & bt_message).await.expect("Error while transferring the data");
        transfer_button.set_sensitive(true);
        }));
    });
// transfer_button.connect_clicked(move |_| { Command::new("python").arg("/Users/jogehring/Documents/Informatik/Sicher Programmieren in Rust/led-name-badge-ls32/led-badge-11x44.py").arg(entry.text().as_str()).arg("-s").arg((scale.value() as i32).to_string()).arg("-m").arg(drop_down.selected().to_string()).arg("-b").arg((if flash.is_active() { 1 } else { 0 }).to_string()).spawn().expect("Transfer failed!"); });
    boxed::Box::from(content)
}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_message(entry: &Entry, scale: &Scale, drop_down: &DropDown, flash_button: &ToggleButton, marquee_button: &ToggleButton, invert_button: &ToggleButton) -> Message {
    let texts = vec![String::from(entry.text())];
    let speed = vec![Speed::get(scale.value())];
    let mode = vec![Animation::get(drop_down.selected())];
    let flash = vec![flash_button.is_active()];
    let marquee = vec![marquee_button.is_active()];
    let inverted = vec![invert_button.is_active()];
    Message { texts, inverted, flash, marquee, speed, mode }
}