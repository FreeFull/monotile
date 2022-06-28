#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;

mod ui;
use vizia::prelude::*;

fn main() {
    let app = Application::new(|ctx| {
        ui::build(ctx);
    })
    .title("Monotile")
    .inner_size((300, 300));
    app.run();
}
