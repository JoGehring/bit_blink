use libadwaita::{ApplicationWindow, gtk};
use libadwaita::gdk::Display;
use libadwaita::glib::{clone, MainContext};
use libadwaita::gtk::{Box, CssProvider, DropDown, Entry, Orientation, Scale, style_context_add_provider_for_display, ToggleButton};
use libadwaita::prelude::{AdwApplicationWindowExt, BoxExt,EditableExt, ButtonExt, RangeExt, ToggleButtonExt, WidgetExt};

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

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn build_ui(app_window: &'static ApplicationWindow, text: Option<String>, speed: Option<Speed>, flash: Option<bool>, marquee: Option<bool>, invert: Option<bool>, mode: Option<Animation>) {
    let (input_box, entry) = input_box::build_input_box();
    let (stack_switcher, stack, scale, flash_button, marquee_button, invert_button, drop_down) = view_stack::build_view_stack();
    let (bottom_box, save_button, transfer_button) = bottom_box::build_bottom_box();
    let without_header_bar = Box::new(Orientation::Vertical, 0);
    let content = Box::new(Orientation::Vertical, 0);
    without_header_bar.append(input_box.as_ref());
    without_header_bar.append(stack_switcher.as_ref());
    without_header_bar.append(stack.as_ref());
    without_header_bar.append(bottom_box.as_ref());

    if text.is_some(){
        entry.set_text(text.unwrap().as_str());
    }
    if speed.is_some(){
        scale.set_value(Speed::get_value(speed.unwrap()));
    }
    if flash.is_some(){
        flash_button.set_active(flash.unwrap());
    }
    if marquee.is_some(){
        marquee_button.set_active(marquee.unwrap());
    }    
    if invert.is_some(){
        invert_button.set_active(invert.unwrap());
    }
    if mode.is_some(){
        drop_down.set_selected(Animation::get_value(mode.unwrap()));
    }

    let (header_bar, delete_buttons) = message_list::get_message_list(entry, scale, flash_button, marquee_button, invert_button, drop_down);

    content.append(&header_bar);
    content.append(&without_header_bar);
    save_button.connect_clicked(move |save_button| {
        save_button.set_sensitive(false);
        let mut bt_message = build_message(entry, scale, drop_down, flash_button, marquee_button, invert_button);
        let msg_storage = build_storage();
        msg_storage.save_message(&mut bt_message);
        build_ui(app_window, Some(bt_message.texts[0].clone()), Some(bt_message.speed[0].clone()), Some(bt_message.flash[0]), Some(bt_message.marquee[0]), Some(bt_message.inverted[0]), Some(bt_message.mode[0].clone()));
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
            build_ui(app_window, None, None, None, None, None, None);
        });
    }
// transfer_button.connect_clicked(move |_| { Command::new("python").arg("/Users/jogehring/Documents/Informatik/Sicher Programmieren in Rust/led-name-badge-ls32/led-badge-11x44.py").arg(entry.text().as_str()).arg("-s").arg((scale.value() as i32).to_string()).arg("-m").arg(drop_down.selected().to_string()).arg("-b").arg((if flash.is_active() { 1 } else { 0 }).to_string()).spawn().expect("Transfer failed!"); });
    app_window.set_content(Some(&content));
    app_window.show();
}

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
mod tests{
    #[test]
    fn save_message_test() {
        let i = 3;
        assert_eq!(3, i);
    }
}