use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use gdk::prelude::*;
use gdk::{EventButton, EventMask, EventMotion, ModifierType};

use gio::prelude::*;

use crate::ui::state::{State, Tool};

const SCALE: f64 = 2.0;

pub fn build(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = state.canvas_surface.borrow().canvas.size();
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32 * 2, height as i32 * 2);
    let mask = EventMask::POINTER_MOTION_MASK
        | EventMask::BUTTON_PRESS_MASK
        | EventMask::LEAVE_NOTIFY_MASK;
    area.add_events(mask);
    area.connect_draw({
        let state = state.clone();
        move |_, cr| {
            cr.scale(SCALE, SCALE);
            state.canvas_surface.borrow().draw(&cr);
            if let Some((x, y)) = state.canvas_cursor_position.borrow().clone() {
                let tile = state.current_tile.borrow().clone();
                state
                    .canvas_surface
                    .borrow()
                    .tileset
                    .draw_tile_alpha(&cr, x, y, &tile, 0.5);
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
    let tool = state.current_tool.borrow().clone();
    match tool {
        Tool::Draw => draw(area, state, position, modifiers),
        Tool::FloodFill => fill(area, state, position, modifiers),
    }
    Inhibit(true)
}

fn moved(area: &gtk::DrawingArea, state: &Rc<State>, event: &EventMotion) -> Inhibit {
    event.request_motions();
    let position = event.get_position();
    let modifiers = event.get_state();
    if *state.current_tool.borrow() == Tool::Draw {
        draw(area, state, position, modifiers);
    }
    Inhibit(true)
}

fn left(area: &gtk::DrawingArea, state: &Rc<State>) -> Inhibit {
    state.canvas_cursor_position.replace(None);
    area.queue_draw();
    Inhibit(false)
}

fn draw(area: &gtk::DrawingArea, state: &Rc<State>, position: (f64, f64), modifiers: ModifierType) {
    let (x, y) = position;
    let (x, y) = (x / (8.0 * SCALE), y / (8.0 * SCALE));
    let tile = state.current_tile.borrow().clone();
    if modifiers.contains(ModifierType::BUTTON1_MASK) {
        state
            .canvas_surface
            .borrow_mut()
            .set_tile(x as usize, y as usize, tile);
        state.app.activate_action("modified", None);
    } else if modifiers.contains(ModifierType::BUTTON3_MASK) {
        let tile = state
            .canvas_surface
            .borrow()
            .canvas
            .get_tile(x as usize, y as usize);
        state.current_tile.replace(tile);
        state
            .window
            .get_window()
            .map(|window| window.invalidate_rect(None, true));
    }
    let (max_x, max_y) = state.canvas_surface.borrow().canvas.size();
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

fn fill(
    area: &gtk::DrawingArea,
    state: &Rc<State>,
    position: (f64, f64),
    _modifiers: ModifierType,
) {
    let (x, y) = position;
    let (x, y) = (x / (8.0 * SCALE), y / (8.0 * SCALE));
    let tile = state.current_tile.borrow().clone();
    state
        .canvas_surface
        .borrow_mut()
        .flood_fill(x as usize, y as usize, tile);
    state.app.activate_action("modified", None);
    area.queue_draw();
}
