use libadwaita::gtk::{Box, Grid, Label, ListBox, Orientation, PositionType, Scrollable, ScrolledWindow, Separator, Switch};
use libadwaita::prelude::{BoxExt, GridExt};

use crate::storage::message_provider::get_all_messages;
use crate::ui::message_list;

pub fn get_message_list() -> ScrolledWindow {
    let mut row = 0;
    let height = 1;
    let width = 1;
    let grid = Grid::builder().build();
    let number_label = Label::builder().label("#").css_classes(["grid_header"]).build();
    grid.attach(&number_label, 0, row, width, height);
    let message_label = Label::builder().label("Message").css_classes(["grid_header"]).build();
    grid.attach_next_to(&message_label, Some(&number_label), PositionType::Right, width, height);
    let speed_label = Label::builder().label("Speed").css_classes(["grid_header"]).build();
    grid.attach_next_to(&speed_label, Some(&message_label), PositionType::Right, width, height);
    let flash_label = Label::builder().label("Flash").css_classes(["grid_header"]).build();
    grid.attach_next_to(&flash_label, Some(&speed_label), PositionType::Right, width, height);
    let marquee_label = Label::builder().label("Marquee").css_classes(["grid_header"]).build();
    grid.attach_next_to(&marquee_label, Some(&flash_label), PositionType::Right, width, height);
    let invert_label = Label::builder().label("Invert").css_classes(["grid_header"]).build();
    grid.attach_next_to(&invert_label, Some(&marquee_label), PositionType::Right, width, height);
    let animation_label = Label::builder().label("Animation").css_classes(["grid_header"]).build();
    grid.attach_next_to(&animation_label, Some(&invert_label), PositionType::Right, width, height);
    let active_label = Label::builder().label("Active?").css_classes(["grid_header"]).build();
    grid.attach_next_to(&active_label, Some(&animation_label), PositionType::Right, width, height);

    for message in get_all_messages() {
        row += 1;
        let number = Label::builder().label((row/2+1).to_string()).css_classes(["grid_item", "number"]).build();
        grid.attach(&number, 0, row, width, height);
        grid.attach_next_to(&Separator::new(Orientation::Vertical), Some(&number), PositionType::Right, width, height);
        let text = Label::builder().label(&message.texts[0]).css_classes(["grid_item"]).build();
        grid.attach_next_to(&text, Some(&number), PositionType::Right, width, height);
        let speed = Label::builder().label(&message.speed[0].to_string()).css_classes(["grid_item"]).build();
        grid.attach_next_to(&speed, Some(&text), PositionType::Right, width, height);
        let flash = Label::builder().label(&message.flash[0].to_string()).css_classes(["grid_item"]).build();
        grid.attach_next_to(&flash, Some(&speed), PositionType::Right, width, height);
        let marquee = Label::builder().label(&message.marquee[0].to_string()).css_classes(["grid_item"]).build();
        grid.attach_next_to(&marquee, Some(&flash), PositionType::Right, width, height);
        let invert = Label::builder().label(&message.inverted[0].to_string()).css_classes(["grid_item"]).build();
        grid.attach_next_to(&invert, Some(&marquee), PositionType::Right, width, height);
        let animation = Label::builder().label(&message.mode[0].to_string()).css_classes(["grid_item"]).build();
        grid.attach_next_to(&animation, Some(&invert), PositionType::Right, width, height);
        let is_active = Switch::builder().hexpand_set(false).vexpand_set(false).build();
        grid.attach_next_to(&is_active, Some(&animation), PositionType::Right, width, height);
        row += 1;
        let separator = Separator::new(Orientation::Horizontal);
        grid.attach(&separator, 0, row, 8, 1);
    }
    ScrolledWindow::builder().child(&grid).can_focus(true).build()
}