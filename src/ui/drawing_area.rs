use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use ui::State;

pub fn build(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = state.canvas.borrow().size();
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32 * 2, height as i32 * 2);
    area.connect_draw({
        let state = state.clone();
        move |_, cr| {
            cr.scale(2.0, 2.0);
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

    area.show_all();
    area
}
