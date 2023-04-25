use libadwaita::ActionRow;
use libadwaita::glib::IsA;
use libadwaita::gtk::Widget;
use libadwaita::prelude::ActionRowExt;

pub fn build_effects_page() -> impl IsA<Widget> {
    let button = ActionRow::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .title("Click me")
        .activatable(true)
        .build();
    button.connect_activated(|_| {
        eprintln!("Submitted!");
    });
    button
}