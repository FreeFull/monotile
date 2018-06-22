use gtk;
use gtk::prelude::*;

use ui::canvas::{Canvas, Tile};
use ui::State;

pub fn build(state: &State) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = state.canvas.borrow().size();
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32, height as i32);
    area.connect_draw({
        let state = state.clone();
        move |_, cr| {
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.paint();
            cr.set_source_rgb(1.0, 1.0, 1.0);
            for y in 0..height/8 {
                for x in 0..width/8 {
                    if (x^y) & 1 == 0 {
                        cr.rectangle(x as f64 * 8.0, y as f64 * 8.0, 8.0, 8.0);
                        cr.fill();
                    }
                }
            }
            Inhibit(false)
        }
    });

    area.show_all();
    area
}
