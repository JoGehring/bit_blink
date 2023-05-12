use libadwaita::{Application, gtk, HeaderBar};
use libadwaita::gdk::Display;
use libadwaita::glib::{clone, IsA, MainContext, PRIORITY_DEFAULT};
use libadwaita::gtk::{Box, CssProvider, Orientation, StyleContext, Widget};
use libadwaita::prelude::{BoxExt, ButtonExt, EditableExt, RangeExt, ToggleButtonExt, WidgetExt};

use crate::bluetooth::{Animation, Message, Speed};
use crate::bluetooth::connection;

mod view_stack;
pub mod window;
mod speed_page;
mod effects_page;
mod entry_box;
mod animations_page;
mod bottom_box;


pub fn build_ui() -> Box {
    let (entry_box, entry) = entry_box::build_entry_box();
    let (stack_switcher, stack, scale, flash_button, marquee_button, invert_button, drop_down) = view_stack::build_view_stack();
    let (bottom_box, transfer_button) = bottom_box::build_bottom_box(&entry);
    let content = Box::new(Orientation::Vertical, 0);
    content.append(&HeaderBar::new());
    content.append(&entry_box);
    content.append(&stack_switcher);
    content.append(&stack);
    content.append(&bottom_box);
    //Generate Message


    //convert Message in the write format

    transfer_button.connect_clicked(move |transfer_button| {
        let main_context = MainContext::default();
        // The main loop executes the asynchronous block
        main_context.spawn_local(clone!(@strong entry, @strong scale, @strong drop_down, @strong flash_button, @strong marquee_button, @strong invert_button, @strong transfer_button => async move {
                transfer_button.set_sensitive(false);
                let speed = vec![Speed::get(scale.value())];
                let mode = vec![Animation::get(drop_down.selected())];
                let flash = vec![flash_button.is_active()];
                let marquee = vec![marquee_button.is_active()];
                let inverted = vec![invert_button.is_active()];
                let bt_message = Message{texts: vec![String::from(entry.text())], inverted, flash, marquee, speed, mode, test: vec![] };
                connection(&bt_message).await.expect("Error while transferring the data");
                transfer_button.set_sensitive(true);
        }));
    });
// transfer_button.connect_clicked(move |_| { Command::new("python").arg("/Users/jogehring/Documents/Informatik/Sicher Programmieren in Rust/led-name-badge-ls32/led-badge-11x44.py").arg(entry.text().as_str()).arg("-s").arg((scale.value() as i32).to_string()).arg("-m").arg(drop_down.selected().to_string()).arg("-b").arg((if flash.is_active() { 1 } else { 0 }).to_string()).spawn().expect("Transfer failed!"); });
    content
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