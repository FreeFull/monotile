use std::rc::Rc;

use gtk::prelude::*;
use gtk::{ColorButton, Grid, Label};

use gdk;
use gdk::prelude::*;

use ui::State;

pub fn build(state: &Rc<State>) -> Grid {
    let tile = *state.current_tile.borrow();
    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    let fg_label = Label::new("fg");
    let fg_button = ColorButton::new_with_rgba(&tile.fg.into());
    fg_button.connect_color_set({
        let state = state.clone();
        move |button| {
            state.current_tile.borrow_mut().fg = button.get_rgba().into();
            button
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
        }
    });
    let bg_label = Label::new("bg");
    let bg_button = ColorButton::new_with_rgba(&gdk::RGBA::from(tile.bg));
    bg_button.connect_color_set({
        let state = state.clone();
        move |button| {
            state.current_tile.borrow_mut().bg = button.get_rgba().into();
            button
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
        }
    });
    grid.attach(&fg_label, 0, 0, 1, 1);
    grid.attach(&fg_button, 1, 0, 1, 1);
    grid.attach(&bg_label, 0, 1, 1, 1);
    grid.attach(&bg_button, 1, 1, 1, 1);
    grid
}
