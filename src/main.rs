#![windows_subsystem = "windows"]

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

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
