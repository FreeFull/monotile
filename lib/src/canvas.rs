use std::default::Default;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            tiles: vec![Tile::default(); width as usize * height as usize],
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn tiles(&self) -> impl Iterator<Item = (u32, u32, &Tile)> {
        let width = self.width;
        self.tiles.iter().enumerate().map(move |(i, tile)| {
            (
                (i % width as usize) as u32,
                (i / width as usize) as u32,
                tile,
            )
        })
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Tile {
        self.tiles[x as usize + y as usize * self.width as usize]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.tiles[x as usize + y as usize * self.width as usize] = tile;
    }

    pub fn flood_fill(&mut self, mut x: u32, mut y: u32, tile: Tile) {
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
        assert_eq!(tiles.len(), self.width as usize * self.height as usize);
        self.tiles = tiles;
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::SetTile { x, y, tile } => self.set_tile(x, y, tile),
            Action::FloodFill { x, y, tile } => self.flood_fill(x, y, tile),
        }
    }
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas::new(32, 32)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Tile {
    pub index: u32,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            index: 32,
            fg: Color::rgb(255, 255, 255),
            bg: Color::rgb(0, 0, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
    pub fn to_argb(&self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        (0xFF << 24) | (r << 16) | (g << 8) | b
    }
}

#[cfg(feature = "libcosmic")]
impl From<Color> for cosmic::iced::Color {
    fn from(color: Color) -> Self {
        Self {
            r: color.r as f32 / u8::MAX as f32,
            g: color.g as f32 / u8::MAX as f32,
            b: color.b as f32 / u8::MAX as f32,
            a: 1.0,
        }
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        image::Rgb([color.r, color.g, color.b])
    }
}

impl From<Color> for image::Rgba<u8> {
    fn from(color: Color) -> Self {
        image::Rgba([color.r, color.g, color.b, 255])
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    SetTile { x: u32, y: u32, tile: Tile },
    FloodFill { x: u32, y: u32, tile: Tile },
}
