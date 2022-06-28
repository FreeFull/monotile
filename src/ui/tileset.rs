use std::fmt;

use crate::ui::canvas::Tile;

const TILESET_IMAGE: &'static [u8] = include_bytes!("../../data/tiles.gif");
pub const WIDTH: u8 = 16;
pub const HEIGHT: u8 = 16;

pub struct Tileset {}

impl fmt::Debug for Tileset {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("<Tileset>")
    }
}

impl Tileset {
    pub fn new() -> Tileset {
        let image = image::load_from_memory(TILESET_IMAGE).unwrap();
        Tileset {}
    }

    #[inline]
    pub fn draw_tile(&self, x: usize, y: usize, tile: &Tile) {}

    pub fn draw_tile_alpha(&self, x: usize, y: usize, tile: &Tile, alpha: f64) {
        self.draw_tile(x, y, tile);
    }
}
