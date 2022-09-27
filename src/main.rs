#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;

mod ui;
use std::env::args_os;
use std::path::PathBuf;

use vizia::prelude::*;

fn main() {
    let path = args_os().nth(1).map(PathBuf::from);
    let app = Application::new(move |cx| {
        cx.add_theme(include_str!("default_style.css"));
        #[cfg(debug_assertions)]
        let _ = cx.add_stylesheet("src/default_style.css");
        ui::build(cx, path.as_ref().map(|path| &**path));
    })
    .min_inner_size(Some((600, 600)));
    app.run();
}
