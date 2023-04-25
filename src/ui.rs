use libadwaita::{Application, HeaderBar};
use libadwaita::gtk::{Box, Orientation};
use libadwaita::prelude::{BoxExt, WidgetExt};

mod view_stack;
mod window;
mod speed_page;
mod effects_page;

pub fn build_ui(app: &Application) {
    let (stack_switcher, stack) = view_stack::build_view_stack();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&HeaderBar::new());
    content.append(&stack_switcher);
    content.append(&stack);

    let window = window::create_window(&app, &content);
    window.show();
}