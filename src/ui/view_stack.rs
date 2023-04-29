use libadwaita::ActionRow;
use libadwaita::gtk::{Box, Entry, Orientation, Scale, Stack, StackSwitcher};
use libadwaita::gtk::StackTransitionType::SlideLeftRight;
use libadwaita::prelude::{ActionRowExt, BoxExt, RangeExt};
use crate::ui::{speed_page, animations_page, effects_page};

pub fn build_view_stack() -> (StackSwitcher, Stack, Scale) {
    let (speed, scale) = speed_page::build_speed_page();
    let effects_page = effects_page::build_effects_page();
    let animations_page = animations_page::build_animations_page();
    let stack_switcher = StackSwitcher::builder().css_classes(["stack"]).margin_top(10).margin_bottom(10).margin_start(10).margin_end(10).build();
    let stack = Stack::builder().build();
    let page1 = stack.add_titled(&speed, Option::<&str>::None, "Speed");
    let page2 = stack.add_titled(&effects_page, Option::<&str>::None, "Effects");
    let page3 = stack.add_titled(&animations_page, Option::<&str>::None, "Animations");
    stack.set_transition_type(SlideLeftRight);
    stack.set_transition_duration(200);
    stack_switcher.set_stack(Option::from(&stack));
    (stack_switcher, stack, scale)
}