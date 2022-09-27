use std::path::Path;
use std::path::PathBuf;
use vizia::prelude::*;

use super::actions::*;
use super::canvas;
use super::canvas::{Canvas, Tile};
use super::tileset::Tileset;

pub fn build(file: Option<impl AsRef<Path>>, cx: &mut Context) {
    State::new(file, cx);
}

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
    pub fn new(file: Option<impl AsRef<Path>>, cx: &mut Context) {
        use super::file_formats;
        let state = match (move || -> std::io::Result<State> {
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
        };
        cx.emit(WindowEvent::SetTitle(state.title()));
        state.build(cx);
    }

    pub fn title(&self) -> String {
        format!(
            "Monotile - {}{}",
            if self.modified { "*" } else { "" },
            if let Some(ref name) = self.file_name {
                name
            } else {
                "Untitled"
            }
        )
    }
}

impl Model for State {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action: &Action, _metadata| {
            match action {
                Action::New => todo!(),
                Action::Open => todo!(),
                Action::Save => {
                    let _ = save(self);
                    cx.emit(WindowEvent::SetTitle(self.title()));
                }
                Action::SaveAs => todo!(),
                Action::Quit => {
                    if !self.modified {
                        std::process::exit(0);
                    } else {
                        cx.emit(super::dialogs::StateSetter::Quit(true));
                    }
                }
                Action::Undo => todo!(),
                Action::Redo => todo!(),
                Action::Copy => todo!(),
                Action::Cut => todo!(),
                Action::Paste => todo!(),
                Action::Help => todo!(),
                Action::About => todo!(),
                Action::TileUp => {
                    self.current_tile.index =
                        self.current_tile.index.saturating_sub(self.tileset.width);
                }
                Action::TileLeft => {
                    self.current_tile.index = self.current_tile.index.saturating_sub(1);
                }
                Action::TileDown => {
                    self.current_tile.index = self
                        .current_tile
                        .index
                        .saturating_add(self.tileset.width)
                        .min((self.tileset.width as u64 * self.tileset.height as u64 - 1) as u32);
                }
                Action::TileRight => {
                    self.current_tile.index =
                        self.current_tile.index.saturating_add(1).min(
                            (self.tileset.width as u64 * self.tileset.height as u64 - 1) as u32,
                        );
                }
                Action::TileIndex(index) => {
                    self.current_tile.index = *index as u32;
                }
            };
            cx.emit(WindowEvent::SetTitle(self.title()));
        });
        event.map(|tool: &Tool, _metadata| {
            self.current_tool = *tool;
        });
        event.map(|action: &canvas::Action, _| {
            self.canvas.handle_action(*action);
            self.modified = true;
            cx.emit(WindowEvent::SetTitle(self.title()));
        });
        event.map(|message: &WindowEvent, meta| match *message {
            WindowEvent::WindowClose => {
                println!("Window close detected!");
                meta.consume();
                //cx.emit(Action::Quit);
            }
            _ => {}
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
