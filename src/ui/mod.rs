use std::path::Path;

pub use self::state::State;
use self::views::*;
use vizia::prelude::*;

mod actions;
mod canvas;
mod dialogs;
mod file_formats;
mod keymap;
mod menu;
mod state;
mod tileset;
mod views;

pub fn build(cx: &mut Context, file: Option<impl AsRef<Path>>) {
    state::build(file, cx);
    keymap::build(cx);
    menu::build(cx);
    dialogs::build(cx);
    HStack::new(cx, |cx| {
        drawing_area::build(cx);
        VStack::new(cx, |cx| {
            tile_chooser::build(cx);
            color_chooser::build(cx);
            tool_chooser::build(cx);
        })
        .width(Units::Auto);
    });
}
