use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

use serde_json;

use super::canvas::{Canvas, Color, Tile};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
    pub foreground: Vec<u32>,
    pub background: Vec<u32>,
}

impl SaveData {
    fn tiles(&self) -> Vec<Tile> {
        let mut tiles = vec![];
        for ((&index, &fg), &bg) in self
            .tiles
            .iter()
            .zip(self.foreground.iter())
            .zip(self.background.iter())
        {
            let fg = Color::rgba(
                (fg >> 16 & 0xFF) as u8,
                (fg >> 8 & 0xFF) as u8,
                (fg & 0xFF) as u8,
                (fg >> 24 & 0xFF) as u8,
            );
            let bg = Color::rgba(
                (bg >> 16 & 0xFF) as u8,
                (bg >> 8 & 0xFF) as u8,
                (bg & 0xFF) as u8,
                (bg >> 24 & 0xFF) as u8,
            );
            tiles.push(Tile { index, fg, bg });
        }
        tiles
    }
}

pub fn save<P: AsRef<Path>>(path: P, canvas: &Canvas) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    let (width, height) = canvas.size();
    let mut tiles = vec![];
    let mut foreground = vec![];
    let mut background = vec![];
    for (_, _, tile) in canvas.tiles() {
        tiles.push(tile.index);
        foreground.push(tile.fg.to_argb());
        background.push(tile.bg.to_argb());
    }
    let save_data = SaveData {
        width,
        height,
        tiles,
        foreground,
        background,
    };
    serde_json::to_writer(file, &save_data)?;
    Ok(())
}

pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Canvas> {
    let file = File::open(path)?;
    let save_data: SaveData = serde_json::from_reader(file)?;
    let mut canvas = Canvas::new(save_data.width, save_data.height);
    canvas.set_all_tiles(save_data.tiles());
    Ok(canvas)
}
