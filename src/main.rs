#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;

use gio::prelude::*;
use gio::ApplicationFlags;

mod ui;

fn main() {
    let app = gtk::Application::new(
        Some("com.github.freefull.monotile"),
        ApplicationFlags::HANDLES_OPEN | ApplicationFlags::NON_UNIQUE,
    );
    app.connect_startup(ui::build);
    app.connect_activate(|_| ());
    app.run();
}
