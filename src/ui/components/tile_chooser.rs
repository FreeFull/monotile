use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use gdk::{EventButton, EventMask};

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
                state.tileset.draw_tile(&cr, x, y, &tile);
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

    area.show_all();
    area
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
    tile.index = x + y*16;
    area.queue_draw();
    Inhibit(true)
}
