use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use gdk::prelude::*;
use gdk::{EventButton, EventMask, EventMotion, ModifierType};

use ui::State;

const SCALE: f64 = 2.0;

pub fn build(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = state.canvas.borrow().size();
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32 * 2, height as i32 * 2);
    let mask = EventMask::POINTER_MOTION_MASK
        | EventMask::BUTTON_PRESS_MASK
        | EventMask::LEAVE_NOTIFY_MASK;
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
            if let Some((x, y)) = state.canvas_cursor_position.borrow().clone() {
                let tile = state.current_tile.borrow().clone();
                state.tileset.draw_tile_alpha(&cr, x, y, &tile, 0.5);
            }
            Inhibit(false)
        }
    });

    area.connect_button_press_event({
        let state = state.clone();
        move |area, event| pressed(area, &state, event)
    });

    area.connect_motion_notify_event({
        let state = state.clone();
        move |area, event| moved(area, &state, event)
    });

    area.connect_leave_notify_event({
        let state = state.clone();
        move |area, _| left(area, &state)
    });

    area.show_all();
    area
}

fn pressed(area: &gtk::DrawingArea, state: &Rc<State>, event: &EventButton) -> Inhibit {
    let position = event.get_position();
    let mut modifiers = event.get_state();
    // The button doesn't get included in the state for EventButton
    modifiers |= match event.get_button() {
        1 => ModifierType::BUTTON1_MASK,
        2 => ModifierType::BUTTON2_MASK,
        3 => ModifierType::BUTTON3_MASK,
        _ => ModifierType::empty(),
    };
    mouse_event(area, state, position, modifiers);
    Inhibit(true)
}

fn mouse_event(
    area: &gtk::DrawingArea,
    state: &Rc<State>,
    position: (f64, f64),
    modifiers: ModifierType,
) {
    let (x, y) = position;
    let (x, y) = (x / (8.0 * SCALE), y / (8.0 * SCALE));
    let tile = state.current_tile.borrow().clone();
    if modifiers.contains(ModifierType::BUTTON1_MASK) {
        state
            .canvas
            .borrow_mut()
            .set_tile(x as usize, y as usize, tile);
    } else if modifiers.contains(ModifierType::BUTTON3_MASK) {
        let tile = state.canvas.borrow().get_tile(x as usize, y as usize);
        state.current_tile.replace(tile);
        state
            .window
            .get_window()
            .map(|window| window.invalidate_rect(None, true));
    }
    let (max_x, max_y) = state.canvas.borrow().size();
    let (mut x, mut y) = (x, y);
    x = x.max(0.0);
    x = x.min(max_x as f64);
    y = y.max(0.0);
    y = y.min(max_y as f64);
    state
        .canvas_cursor_position
        .replace(Some((x as usize, y as usize)));
    area.queue_draw();
}

fn moved(area: &gtk::DrawingArea, state: &Rc<State>, event: &EventMotion) -> Inhibit {
    event.request_motions();
    let position = event.get_position();
    let modifiers = event.get_state();
    mouse_event(area, state, position, modifiers);
    Inhibit(true)
}

fn left(area: &gtk::DrawingArea, state: &Rc<State>) -> Inhibit {
    state.canvas_cursor_position.replace(None);
    area.queue_draw();
    Inhibit(false)
}
