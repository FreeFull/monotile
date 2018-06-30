use std::fmt;

use cairo::{self, Pattern, PatternTrait};
use image::{self, GenericImage};

use ui::canvas::Tile;

const TILESET_IMAGE: &'static [u8] = include_bytes!("../../data/tiles.gif");
pub const WIDTH: u8 = 16;
pub const HEIGHT: u8 = 16;

pub struct Tileset {
    pattern: cairo::Pattern,
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
        let pattern = cairo::SurfacePattern::create(&surface);
        pattern.set_filter(cairo::Filter::Nearest);
        Tileset { pattern: Pattern::SurfacePattern(pattern) }
    }

    #[inline]
    pub fn draw_tile(&self, cr: &cairo::Context, x: usize, y: usize, tile: &Tile) {
        cr.save();
        cr.translate(x as f64 * 8.0, y as f64 * 8.0);
        let bg = tile.bg;
        let fg = tile.fg;
        cr.rectangle(0.0, 0.0, 8.0, 8.0);
        cr.clip();
        cr.set_source_rgb(bg.r, bg.g, bg.b);
        cr.paint();
        cr.set_source_rgb(fg.r, fg.g, fg.b);
        cr.translate(
            -((tile.index % WIDTH) as f64) * 8.0,
            -((tile.index / WIDTH) as f64) * 8.0,
        );
        cr.mask(&self.pattern);
        cr.fill();
        cr.restore();
    }
}
