use std::path::Path;
use std::path::PathBuf;
use vizia::prelude::*;

use super::actions::*;
use super::canvas::{Canvas, Tile};
use super::tileset::Tileset;

#[derive(Debug, Lens, Default)]
pub struct State {
    pub file_path: Option<PathBuf>,
    pub file_name: Option<String>,
    pub modified: bool,
    pub current_tile: Tile,
    pub current_tool: Tool,
    pub canvas: Canvas,
    pub tileset: Tileset,
}

impl State {
    pub fn new(file: Option<impl AsRef<Path>>) -> State {
        use super::file_formats;
        match (move || -> std::io::Result<State> {
            if let Some(path) = file {
                let path = path.as_ref();
                let canvas = file_formats::load(&path)?;
                let name = path.file_name().map(|s| s.to_string_lossy().into_owned());
                Ok(State {
                    canvas,
                    file_path: Some(path.to_owned()),
                    file_name: name,
                    ..State::default()
                })
            } else {
                Ok(State::default())
            }
        })() {
            Ok(val) => val,
            Err(_) => State::default(),
        }
    }
}

impl Model for State {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action: &Action, _metadata| match action {
            Action::New => todo!(),
            Action::Save => save(self),
            Action::SaveAs => todo!(),
            Action::Load => todo!(),
        });
        event.map(|tool: &Tool, _metadata| {
            self.current_tool = *tool;
        });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tool {
    Draw,
    FloodFill,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Draw
    }
}
