use std::path::PathBuf;
use vizia::prelude::*;
use vizia::vg;

use super::actions::*;
use super::canvas::{Canvas, Tile};
use super::tileset::{self, Tileset};

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

impl Model for State {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action: &Action, _| match action {
            Action::New => todo!(),
            Action::Save => save(self),
            Action::SaveAs => todo!(),
            Action::Load => todo!(),
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
