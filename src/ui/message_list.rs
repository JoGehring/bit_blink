use libadwaita::gtk::{Button, DropDown, Entry, Grid, Label, MenuButton, Orientation, Popover, PositionType, Scale, ScrolledWindow, Separator, ToggleButton};
use libadwaita::HeaderBar;
use libadwaita::prelude::{ButtonExt, EditableExt, GridExt, RangeExt, ToggleButtonExt, WidgetExt};
use libadwaita::prelude::PopoverExt;

use crate::bluetooth::{Animation, Speed};
use crate::storage::storage::build_storage;

pub fn get_message_list(entry: &'static Entry, scale: &'static Scale, flash_button: &'static ToggleButton, marquee_button: &'static ToggleButton, invert_button: &'static ToggleButton, drop_down: &'static DropDown) -> (HeaderBar, Vec<&'static Button>) {
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
        let number = Label::builder().label((row / 2 + 1).to_string()).css_classes(["number_col"]).build();
        grid.attach(&number, 0, row, 1, 1);
        let v_sep = Separator::new(Orientation::Vertical);
        grid.attach_next_to(&v_sep, Some(&number), PositionType::Right, 1, 1);
        let text = Label::builder().label(&message.texts[0]).css_classes(["grid_item"]).build();
        grid.attach_next_to(&text, Some(&v_sep), PositionType::Right, 5, 1);
        let delete_button = Button::builder().css_classes(["button_header", message.file_name.as_str()]).icon_name("edit-delete").opacity(0.5).build();
        grid.attach_next_to(&delete_button, Some(&text), PositionType::Right, 1, 1);
        let popover_clone = popover.clone();
        let edit_button = Button::builder().css_classes(["button_header"]).icon_name("edit-paste").opacity(0.5).build();
        edit_button.connect_clicked(move |_| {
            entry.set_text(&message.texts[0]);
            scale.set_value(Speed::get_value(message.speed[0].clone()));
            flash_button.set_active(message.flash[0]);
            marquee_button.set_active(message.marquee[0]);
            invert_button.set_active(message.inverted[0]);
            drop_down.set_selected(Animation::get_value(message.mode[0].clone()));
            popover_clone.hide();
        });
        grid.attach_next_to(&edit_button, Some(&delete_button), PositionType::Right, 1, 1);
        row += 1;
        let separator = Separator::new(Orientation::Horizontal);
        grid.attach(&separator, 0, row, 10, 1);
        buttons.push(Box::<Button>::leak(Box::from(delete_button)));
    }
    let message_list = ScrolledWindow::builder().child(&grid).can_focus(true).focus_on_click(true).build();
    popover.set_child(Some(&message_list));
    let list = MenuButton::builder().icon_name("open-menu-symbolic").can_focus(true).focusable(true).focus_on_click(true).popover(&popover).build();
    header_bar.pack_start(&list);
    (header_bar, buttons)
}