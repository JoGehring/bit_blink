use libadwaita::gtk::{DropDown, Scale, Stack, StackSwitcher, ToggleButton};
use libadwaita::gtk::StackTransitionType::SlideLeftRight;
use crate::ui::{speed_page, animations_page, effects_page};

pub fn build_view_stack() -> (Box<StackSwitcher>, Box<Stack>, &'static Scale, &'static ToggleButton, &'static ToggleButton, &'static ToggleButton, &'static DropDown) {
    let (speed, scale) = speed_page::build_speed_page();
    let (effects_page, flash, marquee, invert) = effects_page::build_effects_page();
    let (animations_page, drop_down) = animations_page::build_animations_page();
    let stack_switcher = StackSwitcher::builder().css_classes(["stack"]).build();
    let stack = Stack::builder().build();
    let _page1 = stack.add_titled(speed.as_ref(), Option::<&str>::None, "Speed");
    let _page2 = stack.add_titled(effects_page.as_ref(), Option::<&str>::None, "Effects");
    let _page3 = stack.add_titled(animations_page.as_ref(), Option::<&str>::None, "Animations");
    stack.set_transition_type(SlideLeftRight);
    stack.set_transition_duration(200);
    stack_switcher.set_stack(Option::from(&stack));
    (Box::from(stack_switcher), Box::from(stack), Box::<Scale>::leak(scale), Box::<ToggleButton>::leak(flash), Box::<ToggleButton>::leak(marquee), Box::<ToggleButton>::leak(invert), Box::<DropDown>::leak(drop_down))
}