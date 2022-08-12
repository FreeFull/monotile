#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;

mod ui;
use vizia::prelude::*;
use vizia::state::StaticLens;

fn main() {
    let app = Application::new(|ctx| {
        ui::build(ctx);
    })
    .title("Monotile")
    .min_inner_size(StaticLens::new(&Some((600, 600))));
    app.run();
}
