extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gio::ApplicationFlags;

mod ui;

fn main() {
    let app = gtk::Application::new(
        "com.github.freefull.monotile",
        ApplicationFlags::HANDLES_OPEN | ApplicationFlags::NON_UNIQUE,
    ).unwrap();
    app.connect_startup(ui::build);
    app.connect_activate(|_| ());
    app.run(&std::env::args().collect::<Vec<_>>());
}
