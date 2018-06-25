use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use gdk::{EventButton, EventMask, WindowExt};

use ui::State;

const SCALE: f64 = 2.0;

pub fn build(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = state.canvas.borrow().size();
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32 * 2, height as i32 * 2);
    let mask = EventMask::POINTER_MOTION_MASK | EventMask::BUTTON_PRESS_MASK;
    area.add_events(mask.bits() as i32);
    area.connect_draw({
        let state = state.clone();
        move |_, cr| {
            cr.scale(SCALE, SCALE);
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.rectangle(0.0, 0.0, width as f64, height as f64);
            cr.fill();
            let canvas = state.canvas.borrow();
            for (x, y, tile) in canvas.tiles() {
                state.tileset.draw_tile(&cr, x, y, tile);
            }
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
    let tile = state.current_tile.borrow().clone();
    let (x, y) = event.get_position();
    let (x, y) = (x / (8.0 * SCALE), y / (8.0 * SCALE));
    state
        .canvas
        .borrow_mut()
        .set_tile(x as usize, y as usize, tile);
    if let Some(window) = area.get_window() {
        window.invalidate_rect(None, false);
    }
    Inhibit(true)
}
