extern crate gio;
extern crate gtk;

use gio::prelude::*;

mod ui;

fn main() {
    let app = gtk::Application::new(
        "com.github.freefull.monotile",
        gio::ApplicationFlags::empty(),
    ).unwrap();
    app.connect_startup(|app| ui::build(app));
    app.connect_activate(|_| ());
    app.run(&std::env::args().collect::<Vec<_>>());
}
