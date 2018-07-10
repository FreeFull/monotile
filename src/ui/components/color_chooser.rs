use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, Button, ColorButton, Grid, Label};

use gdk::prelude::*;

use ui::State;

pub fn build(state: &Rc<State>) -> Grid {
    let tile = *state.current_tile.borrow();
    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    let fg_label = Label::new("fg");
    let fg_button = ColorButton::new_with_rgba(&tile.fg.into());
    fg_button.set_tooltip_text("Set foreground colour.");
    fg_button.connect_color_set({
        let state = state.clone();
        move |button| {
            state.current_tile.borrow_mut().fg = button.get_rgba().into();
            state
                .window
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
        }
    });
    fg_button.connect_draw({
        let state = state.clone();
        move |button, _| {
            button.set_rgba(&state.current_tile.borrow().fg.into());
            Inhibit(false)
        }
    });
    let bg_label = Label::new("bg");
    let bg_button = ColorButton::new_with_rgba(&tile.bg.into());
    bg_button.set_tooltip_text("Set background colour.");
    bg_button.connect_color_set({
        let state = state.clone();
        move |button| {
            state.current_tile.borrow_mut().bg = button.get_rgba().into();
            state
                .window
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
        }
    });
    bg_button.connect_draw({
        let state = state.clone();
        move |button, _| {
            button.set_rgba(&state.current_tile.borrow().bg.into());
            Inhibit(false)
        }
    });
    let icon_size = gtk::IconSize::LargeToolbar.into();
    // TODO Implement colour picking
    let pick_fg = Button::new_from_icon_name("color-select-symbolic", icon_size);
    pick_fg.set_tooltip_text("Pick foreground colour. (Unimplemented)");
    let pick_bg = Button::new_from_icon_name("color-select-symbolic", icon_size);
    pick_bg.set_tooltip_text("Pick background colour. (Unimplemented)");
    let swap = Button::new_from_icon_name("media-playlist-shuffle-symbolic", icon_size);
    swap.set_tooltip_text("Swap foreground and background colours.");
    swap.connect_clicked({
        let state = state.clone();
        move |_| {
            let mut tile = state.current_tile.borrow().clone();
            ::std::mem::swap(&mut tile.fg, &mut tile.bg);
            state.current_tile.replace(tile);
            state
                .window
                .get_window()
                .map(|window| window.invalidate_rect(None, true));
        }
    });
    grid.attach(&fg_label, 0, 0, 1, 1);
    grid.attach(&fg_button, 1, 0, 1, 1);
    grid.attach(&pick_fg, 2, 0, 1, 1);

    grid.attach(&bg_label, 0, 1, 1, 1);
    grid.attach(&bg_button, 1, 1, 1, 1);
    grid.attach(&pick_bg, 2, 1, 1, 1);

    grid.attach(&swap, 3, 0, 1, 2);
    grid
}
