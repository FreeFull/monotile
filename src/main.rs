#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;

mod ui;
use std::env::args_os;
use std::path::PathBuf;

use vizia::prelude::*;
use vizia::state::StaticLens;

fn main() {
    let path = args_os().nth(1).map(PathBuf::from);
    let app = Application::new(move |cx| {
        ui::build(cx, path.as_ref().map(|path| &**path));
    })
    .title("Monotile")
    .min_inner_size(StaticLens::new(&Some((600, 600))));
    app.run();
}
