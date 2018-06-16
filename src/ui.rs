use gtk;
use gtk::prelude::*;

pub fn build(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Monotile");
    window.set_default_size(300, 300);

    window.show_all();
}
