
use libadwaita::gtk::{Button, CenterBox,Box, Grid, PositionType};
use libadwaita::prelude::{GridExt, BoxExt};

pub fn get_icon_grid() -> (Box, Vec<Button>) {
    let content = Box::builder()
        .css_classes(["icon_grid"])
        .hexpand_set(true)
        .build();
    let mut row = 0;
    let height = 1;
    let width = 1;
    let grid = Grid::builder().css_classes(["grid"]).hexpand_set(true).vexpand_set(true).build();
    let ball = Button::builder().label("\u{26BD}").css_classes(["icon"]).hexpand_set(true).build();
    let happy1 = Button::builder().label("\u{1F601}").css_classes(["icon"]).hexpand_set(true).build();
    let happy2 = Button::builder().label("\u{1F604}").css_classes(["icon"]).hexpand_set(true).build();
    let heart1 = Button::builder().label("\u{2764}").css_classes(["icon"]).hexpand_set(true).build();
    let heart2 = Button::builder().label("\u{1F495}").css_classes(["icon"]).hexpand_set(true).build();
    let heart3 = Button::builder().label("\u{1F49F}").css_classes(["icon"]).hexpand_set(true).build();
    let heart4 = Button::builder().label("\u{1F497}").css_classes(["icon"]).hexpand_set(true).build();
    //let fablab = Button::builder().label("\u{2699}").css_classes(["icon"]).hexpand_set(true).build();
    let bike = Button::builder().label("\u{1F6B2}").css_classes(["icon"]).hexpand_set(true).build();
    let bike_r = Button::builder().label("\u{1F501}").css_classes(["icon"]).hexpand_set(true).build();
    let owncloud = Button::builder().label("\u{2601}").css_classes(["icon"]).hexpand_set(true).build();
    grid.attach(&ball, 0, row, width, height);
    grid.attach_next_to(&happy1, Some(&ball), PositionType::Right, width, height);
    grid.attach_next_to(&happy2, Some(&happy1), PositionType::Right, width, height);
    grid.attach_next_to(&heart1, Some(&happy2), PositionType::Right, width, height);
    grid.attach_next_to(&heart2, Some(&heart1), PositionType::Right, width, height);
    row += 1;
    grid.attach(&heart3, 0, row, width, height);
    grid.attach_next_to(&heart4, Some(&heart3), PositionType::Right, width, height);
    grid.attach_next_to(&bike, Some(&heart4), PositionType::Right, width, height);
    grid.attach_next_to(&bike_r, Some(&bike), PositionType::Right, width, height);
    grid.attach_next_to(&owncloud, Some(&bike_r), PositionType::Right, width, height);
    let icon_buttons: Vec<Button> = vec![ball, happy1, happy2, heart1, heart2, heart3, heart4, bike, bike_r, owncloud];
    //let icon_buttons: Vec<Button> = vec![ball, happy1, happy2, heart1, heart2, heart3, heart4, fablab, bike, bike_r, owncloud];
    grid.set_column_homogeneous(true);
    content.append(&grid);
    (content, icon_buttons)
}
