use libadwaita::{Application, ApplicationWindow};

/// **Builds the ```ApplicationWindow``` widget**
///
/// # Arguments
///
/// * 'app' - Reference to a ```Application``` object
///
/// * Builds the ```ApplicationWindow``` via the Builder pattern
/// * Connects the ```Application``` to the ```ApplicationWindow``` via reference
///
/// # Returns
/// * A ```ApplicationWindow```
pub fn create_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("BitBlink")
        .resizable(true)
        .build()
}