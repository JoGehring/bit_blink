use std::boxed;
use libadwaita::gtk::{Box, EmojiChooser, Label, MenuButton, Orientation, Popover, PositionType};
use libadwaita::HeaderBar;
use libadwaita::prelude::{BoxExt, PopoverExt, WidgetExt};

use crate::ui::message_list::get_message_list;

pub fn build_header_bar() -> boxed::Box<HeaderBar> {
    let header_bar = HeaderBar::builder().build();
    let popover1 = Popover::builder().position(PositionType::Left).css_classes(["popover"]).can_focus(true).build();
    let message_list = get_message_list();
    popover1.set_child(Some(&message_list));
    let popover2 = Popover::builder().position(PositionType::Left).build();
    let popover_box2 = Box::new(Orientation::Vertical, 5);
    popover_box2.append(&Label::new(Option::from("Hallo, ich bin ein Label im Popover 2")));
    popover2.set_child(Some(&popover_box2));
    let list = MenuButton::builder().icon_name("open-menu-symbolic").popover(&popover1).build();
    let settings = MenuButton::builder().icon_name("system-run-symbolic").popover(&popover2).build();
    header_bar.pack_start(&list);
    header_bar.pack_start(&settings);
    boxed::Box::new(header_bar)
}