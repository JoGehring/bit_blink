

use libadwaita::Application;


use libadwaita::gtk::{Button, Grid, Label, Orientation, PositionType, ScrolledWindow, Separator};
use libadwaita::prelude::{ApplicationExt, ButtonExt, GridExt};


use crate::storage::storage::build_storage;



pub fn get_message_list(app: &Application) -> ScrolledWindow {
    let mut row = 0;
    let v_sep = Separator::new(Orientation::Vertical);

    let grid = Grid::builder().build();
    let number_label = Label::builder().label("#").css_classes(["grid_header"]).build();
    grid.attach(&number_label, 0, row, 1, 1);
    grid.attach_next_to(&v_sep, Some(&number_label), PositionType::Right, 1, 1);
    let message_label = Label::builder().label("Message").css_classes(["grid_header"]).build();
    grid.attach_next_to(&message_label, Some(&v_sep), PositionType::Right, 5, 1);
    // let speed_label = Label::builder().label("Speed").css_classes(["grid_header"]).build();
    // grid.attach_next_to(&speed_label, Some(&message_label), PositionType::Right, width, height);
    // let flash_label = Label::builder().label("Flash").css_classes(["grid_header"]).build();
    // grid.attach_next_to(&flash_label, Some(&speed_label), PositionType::Right, width, height);
    // let marquee_label = Label::builder().label("Marquee").css_classes(["grid_header"]).build();
    // grid.attach_next_to(&marquee_label, Some(&flash_label), PositionType::Right, width, height);
    // let invert_label = Label::builder().label("Invert").css_classes(["grid_header"]).build();
    // grid.attach_next_to(&invert_label, Some(&marquee_label), PositionType::Right, width, height);
    // let animation_label = Label::builder().label("Animation").css_classes(["grid_header"]).build();
    // grid.attach_next_to(&animation_label, Some(&invert_label), PositionType::Right, width, height);
    let delete_label = Label::builder().label("Delete").css_classes(["grid_header"]).build();
    grid.attach_next_to(&delete_label, Some(&message_label), PositionType::Right, 1, 1);
    let edit_label = Label::builder().label("Edit").css_classes(["grid_header"]).build();
    grid.attach_next_to(&edit_label, Some(&delete_label), PositionType::Right, 1, 1);
    let active_label = Label::builder().label("Active").css_classes(["grid_header"]).build();
    grid.attach_next_to(&active_label, Some(&edit_label), PositionType::Right, 1, 1);

    let storage = build_storage();
    let _copy = &storage;
    for message in storage.get_all_messages() {
        row += 1;
        let number = Label::builder().label((row / 2 + 1).to_string()).css_classes(["grid_item", "number"]).build();
        grid.attach(&number, 0, row, 1, 1);
        let v_sep = Separator::new(Orientation::Vertical);
        grid.attach_next_to(&v_sep, Some(&number), PositionType::Right, 1, 1);
        let margin = 200 - (message.texts[0].len() as i32);
        let text = Label::builder().label(&message.texts[0]).css_classes(["grid_item"]).margin_start(20).margin_end(margin).build();
        grid.attach_next_to(&text, Some(&v_sep), PositionType::Right, 5, 1);
        // let speed = Label::builder().label(&message.speed[0].to_string()).css_classes(["grid_item"]).build();
        // grid.attach_next_to(&speed, Some(&text), PositionType::Right, width, height);
        // let flash = Label::builder().label(&message.flash[0].to_string()).css_classes(["grid_item"]).build();
        // grid.attach_next_to(&flash, Some(&speed), PositionType::Right, width, height);
        // let marquee = Label::builder().label(&message.marquee[0].to_string()).css_classes(["grid_item"]).build();
        // grid.attach_next_to(&marquee, Some(&flash), PositionType::Right, width, height);
        // let invert = Label::builder().label(&message.inverted[0].to_string()).css_classes(["grid_item"]).build();
        // grid.attach_next_to(&invert, Some(&marquee), PositionType::Right, width, height);
        // let animation = Label::builder().label(&message.mode[0].to_string()).css_classes(["grid_item"]).build();
        // grid.attach_next_to(&animation, Some(&invert), PositionType::Right, width, height);
        let delete_button = Button::builder().icon_name("edit-delete").opacity(0.5).build();
        let clone = app.clone();
        grid.attach_next_to(&delete_button, Some(&text), PositionType::Right, 1, 1);
        let edit_button = Button::builder().icon_name("edit-paste").opacity(0.5).build();
        edit_button.connect_clicked(move |_button| {
            clone.activate();
        });
        grid.attach_next_to(&edit_button, Some(&delete_button), PositionType::Right, 1, 1);
        // let is_active = Switch::builder().hexpand_set(false).vexpand_set(false).build();
        // grid.attach_next_to(&is_active, Some(&edit_button), PositionType::Right, 1, 1);
        row += 1;
        let separator = Separator::new(Orientation::Horizontal);
        grid.attach(&separator, 0, row, 8, 1);
    }
    ScrolledWindow::builder().child(&grid).can_focus(true).build()
}