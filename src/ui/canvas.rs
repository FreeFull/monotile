use std::default::Default;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Canvas {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            tiles: vec![Tile::default(); width * height],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn tiles(&self) -> impl Iterator<Item = (usize, usize, &Tile)> {
        let width = self.width;
        self.tiles
            .iter()
            .enumerate()
            .map(move |(i, tile)| (i % width, i / width, tile))
    }

    pub fn tiles_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Tile)> {
        let width = self.width;
        self.tiles
            .iter_mut()
            .enumerate()
            .map(move |(i, tile)| (i % width, i / width, tile))
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.tiles[x + y*self.width] = tile;
    }
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas::new(32, 32)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Tile {
    pub index: u8,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b, a: 1.0 }
    }
}
