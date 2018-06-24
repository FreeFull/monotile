use std::fmt;

use cairo;
use image::{self, GenericImage};

use ui::canvas::Tile;

const TILESET_IMAGE: &'static [u8] = include_bytes!("../../data/tiles.gif");

#[derive(Clone)]
pub struct Tileset {
    data: cairo::ImageSurface,
}

impl fmt::Debug for Tileset {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("<Tileset>")
    }
}

impl Tileset {
    pub fn new() -> Tileset {
        let image = image::load_from_memory(TILESET_IMAGE).unwrap();
        let mut surface = cairo::ImageSurface::create(
            cairo::Format::A8,
            image.width() as i32,
            image.height() as i32,
        ).unwrap();
        surface
            .get_data()
            .unwrap()
            .copy_from_slice(&*image.to_luma());
        Tileset { data: surface }
    }

    #[inline]
    pub fn draw_tile(&self, cr: &cairo::Context, x: usize, y: usize, tile: &Tile) {
        cr.save();
        let bg = tile.bg;
        let fg = tile.fg;
        cr.rectangle(x as f64 * 8.0, y as f64 * 8.0, 8.0, 8.0);
        cr.set_source_rgba(bg.r, bg.g, bg.b, bg.a);
        cr.fill();
        cr.set_source_rgba(fg.r, fg.g, fg.b, fg.a);
        cr.restore();
    }
}
