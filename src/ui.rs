use std::boxed;

use libadwaita::{ApplicationWindow, gtk, HeaderBar};
use libadwaita::gdk::Display;
use libadwaita::glib::{clone, MainContext};
use libadwaita::gtk::{Box, Button, CssProvider, DropDown, Entry, Grid, Label, MenuButton, Orientation, Popover, PositionType, Scale, ScrolledWindow, Separator, style_context_add_provider_for_display, ToggleButton};
use libadwaita::prelude::{AdwApplicationWindowExt, BoxExt,EditableExt, ButtonExt, GridExt, PopoverExt, RangeExt, ToggleButtonExt, WidgetExt};

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

    let (header_bar, delete_buttons) = get_message_list(entry, scale, flash_button, marquee_button, invert_button, drop_down);

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

fn get_message_list(entry: &'static Entry, scale: &'static Scale, flash_button: &'static ToggleButton, marquee_button: &'static ToggleButton, invert_button: &'static ToggleButton, drop_down: &'static DropDown) -> (HeaderBar, Vec<&'static Button>) {
    let mut row = 0;
    let v_sep = Separator::new(Orientation::Vertical);
    let header_bar = HeaderBar::new();
    let grid = Grid::builder().build();
    let number_label = Label::builder().label("#").css_classes(["number_col"]).build();
    grid.attach(&number_label, 0, row, 1, 1);
    grid.attach_next_to(&v_sep, Some(&number_label), PositionType::Right, 1, 1);
    let message_label = Label::builder().label("Message").css_classes(["message_col", "message_header"]).build();
    grid.attach_next_to(&message_label, Some(&v_sep), PositionType::Right, 5, 1);
    let delete_label = Label::builder().label("Delete").css_classes(["button_header"]).build();
    grid.attach_next_to(&delete_label, Some(&message_label), PositionType::Right, 1, 1);
    let edit_label = Label::builder().label("Edit").css_classes(["button_header"]).build();
    grid.attach_next_to(&edit_label, Some(&delete_label), PositionType::Right, 1, 1);
    let storage = build_storage();
    let popover = Popover::builder().position(PositionType::Left).css_classes(["popover"]).can_focus(true).build();
    let mut buttons: Vec<&Button> = Vec::new();
    let messages = storage.get_all_messages();

    for message in messages {
        row += 1;
        let flash_clone = flash_button.clone();
        let scale_clone = scale.clone();
        let invert_clone = invert_button.clone();
        let marquee_clone = marquee_button.clone();
        let drop_down_clone = drop_down.clone();
        let entry_clone = entry.clone();
        let popover_clone = popover.clone();
        let number = Label::builder().label((row / 2 + 1).to_string()).css_classes(["number_col"]).build();
        grid.attach(&number, 0, row, 1, 1);
        let v_sep = Separator::new(Orientation::Vertical);
        grid.attach_next_to(&v_sep, Some(&number), PositionType::Right, 1, 1);
        let text = Label::builder().label(&message.texts[0]).css_classes(["grid_item"]).build();
        grid.attach_next_to(&text, Some(&v_sep), PositionType::Right, 5, 1);
        let delete_button = Button::builder().css_classes(["button_header", message.file_name.as_str()]).icon_name("edit-delete").opacity(0.5).build();
        grid.attach_next_to(&delete_button, Some(&text), PositionType::Right, 1, 1);

        let edit_button = Button::builder().css_classes(["button_header"]).icon_name("edit-paste").opacity(0.5).build();
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
        grid.attach(&separator, 0, row, 10, 1);
        buttons.push(boxed::Box::<Button>::leak(boxed::Box::from(delete_button)));
    }
    let message_list = ScrolledWindow::builder().child(&grid).can_focus(true).focus_on_click(true).build();
    popover.set_child(Some(&message_list));
    let list = MenuButton::builder().icon_name("open-menu-symbolic").can_focus(true).focusable(true).focus_on_click(true).popover(&popover).build();
    header_bar.pack_start(&list);
    (header_bar, buttons)
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