use libadwaita::ActionRow;
use libadwaita::gtk::{Box, Entry, Orientation, Scale, Stack, StackSwitcher};
use libadwaita::gtk::StackTransitionType::SlideLeftRight;
use libadwaita::prelude::{ActionRowExt, BoxExt, RangeExt};
use crate::ui::effects_page;
use crate::ui::speed_page;

pub fn build_view_stack() -> (StackSwitcher, Stack) {
    let speed = speed_page::build_speed_page();
    let effects_page = effects_page::build_effects_page();

    let entry = Entry::builder().can_focus(true).focus_on_click(true).margin_top(5).margin_end(5).margin_bottom(5).margin_start(2).hexpand(false).vexpand(false).placeholder_text("Text Input").build();

    let stack_switcher = StackSwitcher::builder().margin_top(10).margin_bottom(10).margin_start(10).margin_end(10).build();
    let stack = Stack::builder().build();
    let page1 = stack.add_titled(&speed, Option::<&str>::None, "Speed");
    let page2 = stack.add_titled(&entry, Option::<&str>::None, "Effects");
    let page3 = stack.add_titled(&effects_page, Option::<&str>::None, "Animations");
    stack.set_transition_type(SlideLeftRight);
    stack.set_transition_duration(200);
    stack_switcher.set_stack(Option::from(&stack));
    (stack_switcher, stack)
}