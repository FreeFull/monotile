use std::cell::RefCell;
use std::path::PathBuf;

use gtk;

use super::canvas::{Canvas, Tile};
use super::tileset;

#[derive(Debug)]
pub struct State {
    pub app: gtk::Application,
    pub window: gtk::ApplicationWindow,
    pub open_file: RefCell<Option<PathBuf>>,
    pub canvas: RefCell<Canvas>,
    pub canvas_cursor_position: RefCell<Option<(usize, usize)>>,
    pub tileset: tileset::Tileset,
    pub current_tile: RefCell<Tile>,
    pub current_tool: RefCell<Tool>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tool {
    Draw,
    FloodFill,
}
