use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::canvas::{Canvas, Color, Tile};
use crate::file_state::FileState;

#[derive(Debug)]
pub struct Handle {
    pub path: PathBuf,
    pub file_type: FileType,
}

impl Handle {
    pub fn name(&self) -> Cow<str> {
        let name = self.path.file_name().unwrap();
        name.to_string_lossy()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum FileType {
    Monti,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u32>,
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
            let fg = Color::rgb(
                (fg >> 16 & 0xFF) as u8,
                (fg >> 8 & 0xFF) as u8,
                (fg & 0xFF) as u8,
            );
            let bg = Color::rgb(
                (bg >> 16 & 0xFF) as u8,
                (bg >> 8 & 0xFF) as u8,
                (bg & 0xFF) as u8,
            );
            tiles.push(Tile { index, fg, bg });
        }
        tiles
    }
}

pub fn save(state: &FileState) -> io::Result<()> {
    let handle = state
        .handle
        .as_ref()
        .expect("FileState file handle missing!");
    let (width, height) = state.canvas.size();
    let mut tiles = vec![];
    let mut foreground = vec![];
    let mut background = vec![];
    for (_, _, tile) in state.canvas.tiles() {
        tiles.push(tile.index);
        foreground.push(tile.fg.to_argb());
        background.push(tile.bg.to_argb());
    }
    let save_data = SaveData {
        width: width as usize,
        height: height as usize,
        tiles,
        foreground,
        background,
    };
    let file = File::create(&handle.path)?;
    serde_json::to_writer(file, &save_data)?;
    Ok(())
}

pub fn load(path: impl AsRef<Path>) -> io::Result<FileState> {
    let path = path.as_ref().to_owned();
    let file = File::open(&path)?;
    let save_data: SaveData = serde_json::from_reader(file)?;
    // TODO: Replace .unwrap() with proper error return
    let mut canvas = Canvas::new(save_data.width.try_into().unwrap(), save_data.height.try_into().unwrap());
    canvas.set_all_tiles(save_data.tiles());
    Ok(FileState {
        canvas,
        handle: Some(Handle {
            path,
            file_type: FileType::Monti,
        }),
        modified: false,
    })
}
