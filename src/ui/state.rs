use std::cell::{Cell, RefCell};
use std::path::PathBuf;

use super::canvas::{Canvas, Tile};
use super::tileset;

#[derive(Debug)]
pub struct State {
    pub app: gtk::Application,
    pub window: gtk::ApplicationWindow,
    pub open_file: RefCell<Option<PathBuf>>,
    pub modified: Cell<bool>,
    pub canvas_surface: RefCell<CanvasSurface>,
    pub canvas_cursor_position: RefCell<Option<(usize, usize)>>,
    pub current_tile: RefCell<Tile>,
    pub current_tool: RefCell<Tool>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tool {
    Draw,
    FloodFill,
}

#[derive(Debug)]
pub struct CanvasSurface {
    pub canvas: Canvas,
    pub surface: cairo::ImageSurface,
    pub tileset: tileset::Tileset,
    _private: (),
}

impl CanvasSurface {
    pub fn new(canvas: Canvas, tileset: tileset::Tileset) -> CanvasSurface {
        let (width, height) = canvas.size();
        let (width, height) = (width as i32 * 8, height as i32 * 8);
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height).unwrap();
        let canvas_surface = CanvasSurface {
            canvas,
            surface,
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
        let cr = cairo::Context::new(&self.surface).unwrap();
        self.tileset.draw_tile(&cr, x, y, &tile);
    }

    pub fn flood_fill(&mut self, x: usize, y: usize, tile: Tile) {
        self.canvas.flood_fill(x, y, tile);
        self.redraw();
    }

    pub fn draw(&self, cr: &cairo::Context) -> Result<(), cairo::Error> {
        cr.save()?;
        let pattern = cairo::SurfacePattern::create(&self.surface);
        pattern.set_filter(cairo::Filter::Nearest);
        cr.set_source(&pattern)?;
        cr.paint()?;
        cr.restore()?;
        Ok(())
    }

    fn redraw(&self) {
        let cr = cairo::Context::new(&self.surface).unwrap();
        let (width, height) = self.canvas.size();
        let (width, height) = (width as f64 * 8.0, height as f64 * 8.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.rectangle(0.0, 0.0, width as f64, height as f64);
        cr.fill().unwrap();
        for (x, y, tile) in self.canvas.tiles() {
            self.tileset.draw_tile(&cr, x, y, tile);
        }
    }
}
