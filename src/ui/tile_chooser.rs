use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use ui::canvas::Tile;
use ui::tileset;
use ui::State;

const SCALE: f64 = 2.0;

pub fn build(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    let (width, height) = (tileset::WIDTH, tileset::HEIGHT);
    let (width, height) = (width * 8, height * 8);
    area.set_size_request(width as i32 * 2 + 4, height as i32 * 2 + 4);
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

    area.show_all();
    area
}
