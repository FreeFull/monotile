use std::default::Default;

use gdk;

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

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.width]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.tiles[x + y * self.width] = tile;
    }

    pub fn flood_fill(&mut self, mut x: usize, mut y: usize, tile: Tile) {
        use std::collections::VecDeque;
        if x >= self.width {
            x = self.width - 1;
        }
        if y >= self.height {
            y = self.height - 1;
        }

        let mut queue = VecDeque::new();
        let old_tile = self.get_tile(x, y);
        if old_tile == tile {
            return;
        }
        queue.push_back((x, y));
        while let Some((x, y)) = queue.pop_front() {
            if self.get_tile(x, y) != old_tile {
                continue;
            }
            self.set_tile(x, y, tile);
            if x == 0 {
                queue.push_back((1, y));
            } else if x == self.width - 1 {
                queue.push_back((x - 1, y));
            } else {
                queue.push_back((x - 1, y));
                queue.push_back((x + 1, y));
            }
            if y == 0 {
                queue.push_back((x, 1));
            } else if y == self.height - 1 {
                queue.push_back((x, y - 1));
            } else {
                queue.push_back((x, y - 1));
                queue.push_back((x, y + 1));
            }
        }
    }

    pub fn set_all_tiles(&mut self, tiles: Vec<Tile>) {
        assert_eq!(tiles.len(), self.width * self.height);
        self.tiles = tiles;
    }
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas::new(32, 32)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Tile {
    pub index: u8,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            index: 32,
            fg: Color::default(),
            bg: Color::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn to_argb(&self) -> u32 {
        let (r, g, b, a) = (self.r as u32, self.g as u32, self.b as u32, self.a as u32);
        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl From<gdk::RGBA> for Color {
    fn from(mut rgba: gdk::RGBA) -> Color {
        rgba.red = rgba.red.min(1.0);
        rgba.red = rgba.red.max(0.0);
        rgba.green = rgba.green.min(1.0);
        rgba.green = rgba.green.max(0.0);
        rgba.blue = rgba.blue.min(1.0);
        rgba.blue = rgba.blue.max(0.0);
        rgba.alpha = rgba.alpha.min(1.0);
        rgba.alpha = rgba.alpha.max(0.0);
        Color {
            r: (rgba.red * 255.0) as u8,
            g: (rgba.green * 255.0) as u8,
            b: (rgba.blue * 255.0) as u8,
            a: (rgba.alpha * 255.0) as u8,
        }
    }
}

impl From<Color> for gdk::RGBA {
    fn from(color: Color) -> gdk::RGBA {
        gdk::RGBA {
            red: color.r as f64 / 255.0,
            green: color.g as f64 / 255.0,
            blue: color.b as f64 / 255.0,
            alpha: color.a as f64 / 255.0,
        }
    }
}
