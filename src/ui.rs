use std::boxed;
use std::cell::RefCell;
use std::rc::Rc;


use libadwaita::{gtk, HeaderBar};
use libadwaita::Application;
use libadwaita::gdk::Display;

use libadwaita::glib::{clone, MainContext, PropertyGet};
use libadwaita::gtk::{Box, Button, CssProvider, DropDown, Entry, Grid, Label, MenuButton, Orientation, Popover, PositionType, Scale, ScrolledWindow, Separator, StyleContext, ToggleButton, style_context_add_provider_for_display};
use libadwaita::prelude::{BoxExt, ButtonExt, CellLayoutExt, EditableExt, GridExt, PopoverExt, RangeExt, ToggleButtonExt, WidgetExt};

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
mod message_list;
mod icon_grid;


pub fn build_ui(_message: Option<&Message>, _app: &Application) -> boxed::Box<Box> {
    let (input_box, entry) = input_box::build_input_box();
    let (stack_switcher, stack, scale, flash_button, marquee_button, invert_button, drop_down) = view_stack::build_view_stack();
    let (bottom_box, save_button, transfer_button) = bottom_box::build_bottom_box();
    let content = Box::new(Orientation::Vertical, 0);

    let without_header_bar = Box::new(Orientation::Vertical, 0);
    without_header_bar.append(input_box.as_ref());
    without_header_bar.append(stack_switcher.as_ref());
    without_header_bar.append(stack.as_ref());
    without_header_bar.append(bottom_box.as_ref());


    let list
        = get_message_list(entry.clone(), scale.clone(), flash_button.clone(), marquee_button.clone(), invert_button.clone(), drop_down.clone());
    let header_bar = HeaderBar::builder().build();

    let settings = get_settings_button();
    header_bar.pack_start(&list);
    header_bar.pack_start(&settings);

    content.append(&header_bar);
    content.append(&without_header_bar);

    let flash_clone = flash_button.clone();
    let scale_clone = scale.clone();
    let invert_clone = invert_button.clone();
    let marquee_clone = marquee_button.clone();
    let drop_down_clone = drop_down.clone();
    let entry_clone = entry.clone();
    let content_clone = content.clone();
    let _messages = header_bar.first_child().unwrap().first_child().unwrap().first_child().unwrap();
    save_button.connect_clicked(move |save_button| {
        save_button.set_sensitive(false);
        let mut bt_message = build_message(&entry_clone, &scale_clone, &drop_down_clone, &flash_clone, &marquee_clone, &invert_clone);
        let msg_storage = build_storage();
        msg_storage.save_message(&mut bt_message);
        content_clone.remove(&header_bar);
        content_clone.remove(&without_header_bar);
        let list
            = get_message_list(entry_clone.clone(), scale_clone.clone(), flash_clone.clone(), marquee_clone.clone(), invert_clone.clone(), drop_down_clone.clone());
        let settings = get_settings_button();
        let header_bar = HeaderBar::builder().build();
        header_bar.pack_start(&list);
        header_bar.pack_start(&settings);
        content_clone.append(&header_bar);
        content_clone.append(&without_header_bar);
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

fn get_settings_button() -> MenuButton {
    let popover2 = Popover::builder().position(PositionType::Left).build();
    let popover_box2 = Box::new(Orientation::Vertical, 5);
    popover_box2.append(&Label::new(Option::from("Hallo, ich bin ein Label im Popover 2")));
    popover2.set_child(Some(&popover_box2));
    let settings = MenuButton::builder().icon_name("system-run-symbolic").popover(&popover2).build();
    settings
}

fn get_message_list(entry: boxed::Box<Entry>, scale: boxed::Box<Scale>, flash_button: boxed::Box<ToggleButton>, marquee_button: boxed::Box<ToggleButton>, invert_button: boxed::Box<ToggleButton>, drop_down: boxed::Box<DropDown>) -> MenuButton {
    let mut row = 0;
    let v_sep = Separator::new(Orientation::Vertical);

    let grid = Grid::builder().build();
    let number_label = Label::builder().label("#").css_classes(["grid_header"]).build();
    grid.attach(&number_label, 0, row, 1, 1);
    grid.attach_next_to(&v_sep, Some(&number_label), PositionType::Right, 1, 1);
    let message_label = Label::builder().label("Message").css_classes(["grid_header"]).margin_end(20).margin_start(20).build();
    grid.attach_next_to(&message_label, Some(&v_sep), PositionType::Right, 5, 1);
    let delete_label = Label::builder().label("Delete").css_classes(["grid_header"]).build();
    grid.attach_next_to(&delete_label, Some(&message_label), PositionType::Right, 1, 1);
    let edit_label = Label::builder().label("Edit").css_classes(["grid_header"]).build();
    grid.attach_next_to(&edit_label, Some(&delete_label), PositionType::Right, 1, 1);
    let active_label = Label::builder().label("Active").css_classes(["grid_header"]).build();
    grid.attach_next_to(&active_label, Some(&edit_label), PositionType::Right, 1, 1);
    let storage = build_storage();
    let popover1 = Popover::builder().position(PositionType::Left).css_classes(["popover"]).can_focus(true).build();
    for message in storage.get_all_messages() {
        row += 1;
        let flash_clone = flash_button.clone();
        let scale_clone = scale.clone();
        let invert_clone = invert_button.clone();
        let marquee_clone = marquee_button.clone();
        let drop_down_clone = drop_down.clone();
        let entry_clone = entry.clone();
        let number = Label::builder().label((row / 2 + 1).to_string()).css_classes(["grid_item", "number"]).build();
        grid.attach(&number, 0, row, 1, 1);
        let v_sep = Separator::new(Orientation::Vertical);
        grid.attach_next_to(&v_sep, Some(&number), PositionType::Right, 1, 1);
        let margin = 200 - (message.texts[0].len() as i32);
        let text = Label::builder().label(&message.texts[0]).css_classes(["grid_item"]).margin_start(20).margin_end(margin).build();
        grid.attach_next_to(&text, Some(&v_sep), PositionType::Right, 5, 1);
        let delete_button = Button::builder().icon_name("edit-delete").opacity(0.5).build();
        grid.attach_next_to(&delete_button, Some(&text), PositionType::Right, 1, 1);
        let edit_button = Button::builder().icon_name("edit-paste").opacity(0.5).build();
        let popover_clone = popover1.clone();
        edit_button.connect_clicked(move |_| {
            entry_clone.set_text(&message.texts[0]);
            scale_clone.set_value(Speed::get_value(message.speed[0].clone()));
            flash_clone.set_active(message.flash[0]);
            marquee_clone.set_active(message.marquee[0]);
            invert_clone.set_active(message.inverted[0]);
            drop_down_clone.set_selected(Animation::get_value(message.mode[0].clone()));
            popover_clone.hide();
        });
        grid.attach_next_to(&edit_button, Some(&delete_button), PositionType::Right, 1, 1);
        row += 1;
        let separator = Separator::new(Orientation::Horizontal);
        grid.attach(&separator, 0, row, 8, 1);
    }
    let message_list = ScrolledWindow::builder().child(&grid).can_focus(true).build();
    popover1.set_child(Some(&message_list));
    let list = MenuButton::builder().icon_name("open-menu-symbolic").popover(&popover1).build();
    list
}

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

fn build_message(entry: &Entry, scale: &Scale, drop_down: &DropDown, flash_button: &ToggleButton, marquee_button: &ToggleButton, invert_button: &ToggleButton) -> Message {
    let texts = vec![String::from(entry.text())];
    let speed = vec![Speed::get(scale.value())];
    let mode = vec![Animation::get(drop_down.selected())];
    let flash = vec![flash_button.is_active()];
    let marquee = vec![marquee_button.is_active()];
    let inverted = vec![invert_button.is_active()];
    Message { file_name: "".to_string(), texts, inverted, flash, marquee, speed, mode }
}