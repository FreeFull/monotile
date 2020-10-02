use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, AccelFlags, AccelGroup, Button, ColorButton, Grid, Label};

use gdk::keys::constants as key;
use gdk::prelude::*;
use gdk::ModifierType;

use crate::ui::State;

pub fn build(state: &Rc<State>) -> Grid {
    let tile = *state.current_tile.borrow();
    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    let fg_label = Label::new(Some("fg"));
    let fg_button = ColorButton::with_rgba(&tile.fg.into());
    fg_button.set_tooltip_markup(Some("Set foreground colour. <b>Q</b>"));
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
    let bg_label = Label::new(Some("bg"));
    let bg_button = ColorButton::with_rgba(&tile.bg.into());
    bg_button.set_tooltip_markup(Some("Set background colour. <b>E</b>"));
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
    let pick_fg = Button::from_icon_name(Some("color-select-symbolic"), icon_size);
    pick_fg.set_tooltip_markup(Some("Pick foreground colour. (Unimplemented) <b>Z</b>"));
    let pick_bg = Button::from_icon_name(Some("color-select-symbolic"), icon_size);
    pick_bg.set_tooltip_markup(Some("Pick background colour. (Unimplemented) <b>C</b>"));
    let swap = Button::from_icon_name(Some("media-playlist-shuffle-symbolic"), icon_size);
    swap.set_tooltip_markup(Some("Swap foreground and background colours. <b>X</b>"));
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

    let group = AccelGroup::new();
    state.window.add_accel_group(&group);

    fg_button.add_accelerator(
        "activate",
        &group,
        *key::Q,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );
    bg_button.add_accelerator(
        "activate",
        &group,
        *key::E,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );
    pick_fg.add_accelerator(
        "activate",
        &group,
        *key::Z,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );
    pick_bg.add_accelerator(
        "activate",
        &group,
        *key::C,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );
    swap.add_accelerator(
        "activate",
        &group,
        *key::X,
        ModifierType::empty(),
        AccelFlags::VISIBLE,
    );

    grid.attach(&fg_label, 0, 0, 1, 1);
    grid.attach(&fg_button, 1, 0, 1, 1);
    grid.attach(&pick_fg, 2, 0, 1, 1);

    grid.attach(&bg_label, 0, 1, 1, 1);
    grid.attach(&bg_button, 1, 1, 1, 1);
    grid.attach(&pick_bg, 2, 1, 1, 1);

    grid.attach(&swap, 3, 0, 1, 2);
    grid
}
