use std::cmp::min;
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use gdk::{EventButton, EventMask};

use gio::prelude::*;
use gio::SimpleAction;

use ui::canvas::Tile;
use ui::tileset;
use ui::State;

const SCALE: f64 = 2.0;

pub fn build(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = (tileset::WIDTH, tileset::HEIGHT);
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32 * 2 + 4, height as i32 * 2 + 4);
    let mask = EventMask::POINTER_MOTION_MASK | EventMask::BUTTON_PRESS_MASK;
    area.add_events(mask.bits() as i32);
    area.connect_draw({
        let state = state.clone();
        move |_, cr| {
            let current_tile = state.current_tile.borrow().clone();
            let fg = current_tile.fg;
            let bg = current_tile.bg;
            cr.scale(SCALE, SCALE);
            cr.translate(1.0, 1.0);
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.paint();
            for i in 0..256 {
                let tile = Tile {
                    index: i as u8,
                    fg,
                    bg,
                };
                let x = i % (tileset::WIDTH as usize);
                let y = i / (tileset::WIDTH as usize);
                state
                    .canvas_surface
                    .borrow()
                    .tileset
                    .draw_tile(&cr, x, y, &tile);
            }
            let i = current_tile.index;
            let x = i % tileset::WIDTH;
            let y = i / tileset::WIDTH;
            cr.rectangle(x as f64 * 8.0 - 0.5, y as f64 * 8.0 - 0.5, 9.0, 9.0);
            cr.set_source_rgb(1.0, 0.0, 0.0);
            cr.set_line_width(1.0);
            cr.stroke();
            Inhibit(false)
        }
    });

    area.connect_button_press_event({
        let state = state.clone();
        move |area, event| pressed(area, &state, event)
    });

    add_actions(state, &area);
    add_accelerators(state);

    area.show_all();
    area
}

fn add_actions(state: &Rc<State>, area: &gtk::DrawingArea) {
    let up = SimpleAction::new("tile.up", None);
    up.connect_activate({
        let state = state.clone();
        let area = area.downgrade();
        move |_, _| {
            {
                let mut tile = state.current_tile.borrow_mut();
                let index = tile.index;
                let (x, mut y) = (index & 0xF, index >> 4);
                y = y.saturating_sub(1);
                tile.index = x | y << 4;
            }
            match area.upgrade() {
                Some(area) => {
                    area.queue_draw();
                }
                None => return,
            }
        }
    });
    let down = SimpleAction::new("tile.down", None);
    down.connect_activate({
        let state = state.clone();
        let area = area.downgrade();
        move |_, _| {
            {
                let mut tile = state.current_tile.borrow_mut();
                let index = tile.index;
                let (x, mut y) = (index & 0xF, index >> 4);
                y = min(y + 1, 15);
                tile.index = x | y << 4;
            }
            match area.upgrade() {
                Some(area) => {
                    area.queue_draw();
                }
                None => return,
            }
        }
    });
    let left = SimpleAction::new("tile.left", None);
    left.connect_activate({
        let state = state.clone();
        let area = area.downgrade();
        move |_, _| {
            {
                let mut tile = state.current_tile.borrow_mut();
                let index = tile.index;
                let (mut x, y) = (index & 0xF, index >> 4);
                x = x.saturating_sub(1);
                tile.index = x | y << 4;
            }
            match area.upgrade() {
                Some(area) => {
                    area.queue_draw();
                }
                None => return,
            }
        }
    });
    let right = SimpleAction::new("tile.right", None);
    right.connect_activate({
        let state = state.clone();
        let area = area.downgrade();
        move |_, _| {
            {
                let mut tile = state.current_tile.borrow_mut();
                let index = tile.index;
                let (mut x, y) = (index & 0xF, index >> 4);
                x = min(x + 1, 15);
                tile.index = x | y << 4;
            }
            match area.upgrade() {
                Some(area) => {
                    area.queue_draw();
                }
                None => return,
            }
        }
    });
    state.app.add_action(&up);
    state.app.add_action(&down);
    state.app.add_action(&left);
    state.app.add_action(&right);
}

fn add_accelerators(state: &Rc<State>) {
    state.app.set_accels_for_action("app.tile.up", &["w"]);
    state.app.set_accels_for_action("app.tile.down", &["s"]);
    state.app.set_accels_for_action("app.tile.left", &["a"]);
    state.app.set_accels_for_action("app.tile.right", &["d"]);
}

fn pressed(area: &gtk::DrawingArea, state: &Rc<State>, event: &EventButton) -> Inhibit {
    let mut tile = state.current_tile.borrow_mut();
    let (x, y) = event.get_position();
    let (mut x, mut y) = ((x - 1.0) / (8.0 * SCALE), (y - 1.0) / (8.0 * SCALE));
    x = x.max(0.0);
    x = x.min(15.0);
    y = y.max(0.0);
    y = y.min(15.0);
    let (x, y) = (x as u8, y as u8);
    tile.index = x + y * 16;
    area.queue_draw();
    Inhibit(true)
}
