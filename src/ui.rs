use std::process::Command;
use std::rc::Rc;
use libadwaita::{Application, gtk, HeaderBar};
use libadwaita::gdk::Display;
use libadwaita::gtk::{Box, CssProvider, Orientation, StyleContext, Widget};
use libadwaita::prelude::{BoxExt, ButtonExt, EditableExt, RangeExt, WidgetExt};

mod view_stack;
mod window;
mod speed_page;
mod effects_page;
mod entry_box;
mod animations_page;
mod bottom_box;

pub fn build_ui(app: &Application) {
    let (entry_box, entry) = entry_box::build_entry_box();
    let (stack_switcher, stack, scale) = view_stack::build_view_stack();
    let (bottom_box, transfer_button) = bottom_box::build_bottom_box(&entry);
    let content = Box::new(Orientation::Vertical, 0);
    content.append(&HeaderBar::new());
    content.append(&entry_box);
    content.append(&stack_switcher);
    content.append(&stack);
    content.append(&bottom_box);
    transfer_button.connect_clicked(move |_| { Command::new("python").arg("/Users/jogehring/Documents/Informatik/Sicher Programmieren in Rust/led-name-badge-ls32/led-badge-11x44.py").arg(entry.text().as_str()).arg("-s").arg((scale.value() as i32).to_string()).spawn().expect("Transfer failed!"); });
    let window = window::create_window(&app, &content);
    window.show();
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