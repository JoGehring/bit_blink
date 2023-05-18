use libadwaita::gdk::pango;
use libadwaita::gtk::{Box, Button, Grid, Image, Label, ListBox, Orientation, PositionType, Scrollable, ScrolledWindow, Separator, Switch};
use libadwaita::prelude::{BoxExt, ButtonExt, GridExt, ObjectExt};

use crate::storage::message_provider::get_all_messages;
use crate::ui::message_list;

pub fn get_icon_grid() -> Grid {
    let mut row = 0;
    let height = 1;
    let width = 1;
    let grid = Grid::builder().margin_start(15).build();
    let ball_image = Image::builder().resource("../images/tree.png").build();
    let ball = Button::builder().label("\u{26BD}").css_classes(["icon"]).build();
    let happy1 = Button::builder().label("\u{1F601}").css_classes(["icon"]).build();
    let happy2 = Button::builder().label("\u{1F604}").css_classes(["icon"]).build();
    let heart1 = Button::builder().label("\u{2764}").css_classes(["icon"]).build();
    let heart2 = Button::builder().label("\u{1F495}").css_classes(["icon"]).build();
    let heart3 = Button::builder().label("\u{1F49F}").css_classes(["icon"]).build();
    let heart4 = Button::builder().label("\u{1F497}").css_classes(["icon"]).build();
    let fablab = Button::builder().label("\u{2699}").css_classes(["icon"]).build();
    let bike = Button::builder().label("\u{1F6B2}").css_classes(["icon"]).build();
    let bike_r = Button::builder().label("\u{1F6B2}\u{1F501}").css_classes(["icon"]).build();
    let owncloud = Button::builder().label("\u{2601}").css_classes(["icon"]).build();
    grid.attach(&ball, 0, row, width, height);
    grid.attach_next_to(&happy1, Some(&ball), PositionType::Right, width, height);
    grid.attach_next_to(&happy2, Some(&happy1), PositionType::Right, width, height);
    grid.attach_next_to(&heart1, Some(&happy2), PositionType::Right, width, height);
    grid.attach_next_to(&heart2, Some(&heart1), PositionType::Right, width, height);
    grid.attach_next_to(&heart3, Some(&heart2), PositionType::Right, width, height);
    row += 1;
    grid.attach(&heart4, 0, row, width, height);
    grid.attach_next_to(&fablab, Some(&heart4), PositionType::Right, width, height);
    grid.attach_next_to(&bike, Some(&fablab), PositionType::Right, width, height);
    grid.attach_next_to(&bike_r, Some(&bike), PositionType::Right, width, height);
    grid.attach_next_to(&owncloud, Some(&bike_r), PositionType::Right, width, height);
    grid.attach_next_to(&ball_image, Some(&owncloud), PositionType::Right, width, height);

    grid
}