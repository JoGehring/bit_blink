use libadwaita::glib::IsA;
use libadwaita::gtk::{CenterBox, Entry};
use libadwaita::gtk::Widget;

pub fn build_entry_box() -> (impl IsA<Widget>, Entry) {
    let entry_box = CenterBox::builder().css_classes(["entry_box"]).build();
    let entry = Entry::builder().can_focus(true).focus_on_click(true).hexpand(true).vexpand(true).placeholder_text("Type your text here...").build();
    entry_box.set_center_widget(Some(&entry));
    (entry_box, entry)
}