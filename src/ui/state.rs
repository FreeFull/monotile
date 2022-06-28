use std::path::PathBuf;
use vizia::prelude::*;

use super::actions::*;
use super::canvas::{Canvas, Tile};
use super::tileset;

#[derive(Debug, Lens)]
pub struct State {
    pub file_path: Option<PathBuf>,
    pub file_name: Option<String>,
    pub modified: bool,
    pub current_tile: Tile,
    pub current_tool: Tool,
    pub canvas: Canvas,
}

impl Model for State {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
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

#[derive(Debug)]
pub struct CanvasSurface {
    pub canvas: Canvas,
    pub tileset: tileset::Tileset,
    _private: (),
}

impl CanvasSurface {
    pub fn new(canvas: Canvas, tileset: tileset::Tileset) -> CanvasSurface {
        let canvas_surface = CanvasSurface {
            canvas,
            tileset,
            _private: (),
        };
        canvas_surface.redraw();
        canvas_surface
    }

    pub fn set_canvas(&mut self, canvas: Canvas) {
        self.canvas = canvas;
        self.redraw();
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.canvas.set_tile(x, y, tile);
        self.tileset.draw_tile(x, y, &tile);
    }

    pub fn flood_fill(&mut self, x: usize, y: usize, tile: Tile) {
        self.canvas.flood_fill(x, y, tile);
        self.redraw();
    }

    pub fn draw(&self) {}

    fn redraw(&self) {
        for (x, y, tile) in self.canvas.tiles() {
            self.tileset.draw_tile(x, y, tile);
        }
    }
}
