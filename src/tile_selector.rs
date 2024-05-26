use crate::Message;
use cosmic::iced::mouse::Interaction;
use cosmic::iced::{Border, Color, Length, Point, Radius, Rectangle, Size, Vector};
use cosmic::iced_core::layout::{Limits, Node};
use cosmic::iced_core::mouse::Cursor;
use cosmic::iced_core::renderer::{Quad, Style};
use cosmic::iced_core::widget::{tree, Tree};
use cosmic::iced_core::{self, image::Renderer as _, Layout, Renderer as _};
use cosmic::iced_core::{event, Event, Shadow, Shell};
use cosmic::widget::image::{FilterMethod, Handle};
use cosmic::widget::Widget;
use cosmic::{Element, Theme};
use image::Pixel;
use libmonotile::canvas::Tile;
use libmonotile::tileset::Tileset;

pub struct TileSelector<'a> {
    current_tile: Tile,
    tileset: &'a Tileset,
    scale: u8,
}

pub fn tile_selector<'a>(current_tile: Tile, tileset: &'a Tileset, scale: u8) -> TileSelector<'a> {
    TileSelector {
        current_tile,
        tileset,
        scale,
    }
}

impl<'a> TileSelector<'a> {
    fn draw_dimensions(&self) -> Size<f32> {
        let scale = self.scale as f32;
        let (width, height) = self.tileset.image.dimensions();
        let mut size = Size::new(width as f32, height as f32);
        // 1 scale*pixel border around the tileset
        size.width += 2.0;
        size.height += 2.0;
        size.width *= scale;
        size.height *= scale;
        size
    }

    fn switch_tile(&self, shell: &mut Shell<'_, Message>, mut position: Point) -> event::Status {
        position.x /= self.scale as f32;
        position.y /= self.scale as f32;
        // Offset to compensate for the border
        position.x = (position.x - 1.0).max(0.0);
        position.y = (position.y - 1.0).max(0.0);
        let position = (position.x as u32, position.y as u32);
        let index = self.tileset.index_from_pixel_position(position);
        let tile = Tile {
            index,
            ..self.current_tile
        };
        if tile.index != self.current_tile.index {
            shell.publish(Message::TileChanged(tile));
        }
        event::Status::Captured
    }
}

impl<'a> Widget<Message, Theme, cosmic::Renderer> for TileSelector<'a> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn size(&self) -> Size<Length> {
        let dimensions = self.draw_dimensions();
        Size::new(
            Length::Fixed(dimensions.width),
            Length::Fixed(dimensions.height),
        )
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &cosmic::Renderer,
        _limits: &Limits,
    ) -> cosmic::iced_core::layout::Node {
        Node::new(self.draw_dimensions())
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut cosmic::Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let mut position = layout.position();
        position.x = position.x.floor();
        position.y = position.y.floor();
        renderer.with_translation(position - Point::ORIGIN, move |renderer| {
            let state = tree.state.downcast_ref::<State>();
            let handle = state.image.clone();
            let mut bounds = layout.bounds();
            bounds.x = 0.0;
            bounds.y = 0.0;
            // Background behind the selector
            renderer.fill_quad(
                Quad {
                    bounds,
                    border: Border::default(),
                    shadow: Shadow::default(),
                },
                Color::BLACK,
            );
            renderer.draw(
                handle,
                FilterMethod::Nearest,
                Rectangle {
                    x: self.scale as f32,
                    y: self.scale as f32,
                    width: self.tileset.image.width() as f32 * self.scale as f32,
                    height: self.tileset.image.height() as f32 * self.scale as f32,
                },
                [0.0; 4],
            );
            let (tile_x, tile_y) = self.tileset.tile_position(self.current_tile.index);
            let tile_x = tile_x * self.tileset.tile_size.0;
            let tile_y = tile_y * self.tileset.tile_size.1;
            let (tile_x, tile_y) = (tile_x as f32, tile_y as f32);
            // Red outline around current tile
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: tile_x * self.scale as f32,
                        y: tile_y * self.scale as f32,
                        width: (self.tileset.tile_size.0 as f32 + 2.0) * self.scale as f32,
                        height: (self.tileset.tile_size.1 as f32 + 2.0) * self.scale as f32,
                    },
                    border: Border {
                        color: Color::new(1.0, 0.0, 0.0, 1.0),
                        radius: Radius::default(),
                        width: self.scale as f32,
                    },
                    shadow: Shadow {
                        ..Default::default()
                    },
                },
                Color::TRANSPARENT,
            );
        });
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &cosmic::Renderer,
    ) -> iced_core::mouse::Interaction {
        if let Some(_position) = cursor.position_over(layout.bounds()) {
            Interaction::Crosshair
        } else {
            Interaction::Idle
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _renderer: &cosmic::Renderer,
        _clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        use iced_core::mouse;
        let state = tree.state.downcast_mut::<State>();
        match event {
            Event::Mouse(event) => match event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    match cursor.position_in(layout.bounds()) {
                        Some(position) => {
                            state.dragging = true;
                            self.switch_tile(shell, position)
                        }
                        None => event::Status::Ignored,
                    }
                }
                mouse::Event::CursorMoved { position } if state.dragging => {
                    let position = Point::ORIGIN + (position - layout.bounds().position());
                    self.switch_tile(shell, position)
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) if state.dragging => {
                    state.dragging = false;
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            },
            Event::Window(_id, event) => match event {
                iced_core::window::Event::RedrawRequested(_) => {
                    state.update_image(self.tileset, self.current_tile);
                    event::Status::Ignored
                }
                _ => event::Status::Ignored,
            },
            _ => event::Status::Ignored,
        }
    }
}

impl<'a> From<TileSelector<'a>> for Element<'a, Message> {
    fn from(value: TileSelector<'a>) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
struct State {
    image: Handle,
    dragging: bool,
    previous_tile: Option<Tile>,
    previous_tileset: Option<Tileset>,
}

impl State {
    pub fn new() -> Self {
        State {
            image: Handle::from_pixels(0, 0, []),
            dragging: false,
            previous_tile: None,
            previous_tileset: None,
        }
    }

    pub fn update_image(&mut self, tileset: &Tileset, current_tile: Tile) {
        // If we already have an up-to-date image, we can just return.
        if self.previous_tile == Some(current_tile)
            && self.previous_tileset.as_ref() == Some(tileset)
        {
            return;
        }
        self.previous_tile = Some(current_tile);
        self.previous_tileset = Some(tileset.clone());

        let mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_pixel(
                tileset.image.width(),
                tileset.image.height(),
                current_tile.bg.into(),
            );
        let mut foreground: image::Rgba<u8> = current_tile.fg.into();
        for (output, &image::Luma([blend])) in image.pixels_mut().zip(tileset.image.pixels()) {
            foreground.0[3] = blend;
            output.blend(&foreground);
        }
        // RGBA
        self.image = Handle::from_pixels(image.width(), image.height(), image.into_raw());
    }
}
