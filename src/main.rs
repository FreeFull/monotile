use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use cosmic::app::{Command, Core, Settings};
use cosmic::iced::keyboard::{self, Key, Modifiers};
use cosmic::iced::{self, Color, Length};
use cosmic::iced_widget::{column, row, scrollable};
use cosmic::widget::menu::{action::MenuAction, key_bind::KeyBind};
use cosmic::widget::{self, container, slider, spin_button, text};
use cosmic::{ApplicationExt, Apply, Element};
use libmonotile::canvas::Tile;
use libmonotile::file_state::FileState;
use libmonotile::tileset::Tileset;

mod actions;
mod key_binds;
mod menu;
mod tile_canvas;
mod tile_selector;

pub use actions::Action;

use tile_canvas::{tile_canvas, State};
use tile_selector::tile_selector;

#[derive(Parser, Debug)]
struct Args {
    path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    cosmic::app::run::<App>(
        Settings::default()
            .client_decorations(true)
            .exit_on_close(false),
        args,
    )?;
    Ok(())
}

struct App {
    core: Core,
    current_tool: Tool,
    current_tile: Tile,
    filename: Option<String>,
    quit_confirmation_dialog: bool,
    tileset: Tileset,
    key_binds: HashMap<KeyBind, Action>,
    modifiers: Modifiers,
    file: libmonotile::file_state::FileState,
    zoom: u8,
}

#[derive(Clone, Debug)]
pub enum Message {
    New,
    Open,
    Save,
    SaveAs,
    CloseRequested,
    Key(Modifiers, Key),
    ModifiersChanged(Modifiers),
    TileChanged(Tile),
    TileUp,
    TileDown,
    TileLeft,
    TileRight,
    ZoomIn,
    ZoomOut,
    CanvasClicked { x: u32, y: u32 },
    Todo,
}

impl cosmic::Application for App {
    const APP_ID: &'static str = "io.github.freefull.monotile";
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = Args;
    type Message = Message;

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = App {
            core,
            current_tool: Tool::Draw,
            current_tile: Tile::default(),
            filename: None,
            quit_confirmation_dialog: false,
            tileset: Tileset::default(),
            key_binds: key_binds::key_binds(),
            modifiers: Modifiers::empty(),
            file: FileState::new(flags.path).unwrap(),
            zoom: 2,
        };
        let command = app.update_title();
        (app, command)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::New => return self.update(Message::Todo),
            Message::Open => return self.update(Message::Todo),
            Message::Save => return self.update(Message::Todo),
            Message::SaveAs => return self.update(Message::Todo),
            Message::CloseRequested => {
                if self.file.modified {
                    self.quit_confirmation_dialog = true;
                } else {
                    return iced::window::close(self.main_window_id());
                }
            }
            Message::Key(modifiers, key) => {
                for (key_bind, action) in self.key_binds.iter() {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }
            }
            Message::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers;
            }
            Message::TileChanged(tile) => {
                self.current_tile = tile;
            }
            Message::TileUp => {
                self.current_tile.index =
                    self.current_tile.index.saturating_sub(self.tileset.width());
            }
            Message::TileDown => {
                self.current_tile.index = (self.current_tile.index + self.tileset.width())
                    .min(self.tileset.width() * self.tileset.height() - 1);
            }
            Message::TileLeft => {
                self.current_tile.index = self.current_tile.index.saturating_sub(1);
            }
            Message::TileRight => {
                self.current_tile.index = (self.current_tile.index + 1)
                    .min(self.tileset.width() * self.tileset.height() - 1);
            }
            Message::ZoomIn => {
                self.zoom = (self.zoom + 1).min(4);
            }
            Message::ZoomOut => {
                self.zoom = (self.zoom - 1).max(1);
            }
            Message::CanvasClicked { x, y } => {
                self.file.canvas.set_tile(x, y, self.current_tile);
                self.file.modified = true;
            }
            Message::Todo => {
                println!("todo");
            }
        };
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::event::listen_with(|event, status| {
            if status != iced::event::Status::Ignored {
                return None;
            }
            match event {
                iced::Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    location: _,
                    modifiers,
                    text: _,
                }) => Some(Message::Key(modifiers, key)),
                iced::Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                    Some(Message::ModifiersChanged(modifiers))
                }
                iced::Event::Window(window_id, event) => {
                    if window_id == cosmic::iced::window::Id::MAIN {
                        match event {
                            iced::window::Event::CloseRequested => Some(Message::CloseRequested),
                            iced::window::Event::FileDropped(_) => Some(Message::Todo),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![menu::menu_bar(&self.key_binds)]
    }

    fn view(&self) -> cosmic::Element<Message> {
        let status_bar = row![spin_button(
            format!("Zoom: {}x", self.zoom),
            |message| match message {
                spin_button::Message::Increment => Message::ZoomIn,
                spin_button::Message::Decrement => Message::ZoomOut,
            }
        )]
        .height(Length::Shrink);
        let fg_color = column![
            "Foreground",
            row![
                spin_button(format!("R: {}", self.current_tile.fg.r), {
                    let tile = self.current_tile;
                    move |message| {
                        let mut tile = tile;
                        match message {
                            spin_button::Message::Increment => {
                                tile.fg.r = tile.fg.r.saturating_add(1);
                                Message::TileChanged(tile)
                            }
                            spin_button::Message::Decrement => {
                                tile.fg.r = tile.fg.r.saturating_sub(1);
                                Message::TileChanged(tile)
                            }
                        }
                    }
                }),
                slider(0..=255, self.current_tile.fg.r, move |r| {
                    let mut tile = self.current_tile;
                    tile.fg.r = r;
                    Message::TileChanged(tile)
                })
            ],
            row![
                text(format!("G: {}", self.current_tile.fg.g)).width(50),
                slider(0..=255, self.current_tile.fg.g, move |g| {
                    let mut tile = self.current_tile;
                    tile.fg.g = g;
                    Message::TileChanged(tile)
                })
            ],
            row![
                text(format!("B: {}", self.current_tile.fg.b)).width(50),
                slider(0..=255, self.current_tile.fg.b, move |b| {
                    let mut tile = self.current_tile;
                    tile.fg.b = b;
                    Message::TileChanged(tile)
                })
            ],
        ];
        let bg_color = column![
            "Background",
            row![
                text(format!("R: {}", self.current_tile.bg.r)).width(50),
                slider(0..=255, self.current_tile.bg.r, move |r| {
                    let mut tile = self.current_tile;
                    tile.bg.r = r;
                    Message::TileChanged(tile)
                })
            ],
            row![
                text(format!("G: {}", self.current_tile.bg.g)).width(50),
                slider(0..=255, self.current_tile.bg.g, move |g| {
                    let mut tile = self.current_tile;
                    tile.bg.g = g;
                    Message::TileChanged(tile)
                })
            ],
            row![
                text(format!("B: {}", self.current_tile.bg.b)).width(50),
                slider(0..=255, self.current_tile.bg.b, move |b| {
                    let mut tile = self.current_tile;
                    tile.bg.b = b;
                    Message::TileChanged(tile)
                })
            ],
        ];
        column![
            row![
                tile_canvas(
                    self.current_tile,
                    &self.tileset,
                    &self.file.canvas,
                    self.zoom
                )
                .apply(container)
                .padding(10)
                .apply(widget::scrollable)
                .direction(scrollable::Direction::Both {
                    vertical: scrollable::Properties::default(),
                    horizontal: scrollable::Properties::default()
                })
                .height(Length::Fill)
                .apply(container)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(cosmic::style::Container::custom(|_| {
                    container::Appearance::default().with_background(Color::BLACK)
                })),
                column![
                    tile_selector(self.current_tile, &self.tileset, 2),
                    fg_color,
                    bg_color
                ]
                .width(Length::Shrink)
            ]
            .height(Length::Fill),
            status_bar
        ]
        .apply(Element::from)
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }
}

impl App {
    fn update_title(&mut self) -> Command<Message> {
        let (header_title, window_title) = {
            let filename = self.filename.as_deref().unwrap_or("Untitled");
            let modified = if self.file.modified { "*" } else { "" };
            (
                format!("{modified}{filename}"),
                format!("{modified}{filename} - App"),
            )
        };
        self.set_header_title(header_title);
        self.set_window_title(window_title)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tool {
    Draw,
    FloodFill,
}
