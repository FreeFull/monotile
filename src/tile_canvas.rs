use crate::Message;
use cosmic::cosmic_theme::palette::angle::FromAngle;
use cosmic::cosmic_theme::palette::cast::ComponentsInto;
use cosmic::iced::{Length, Point, Rectangle, Size};
use cosmic::iced_core::widget::tree;
use cosmic::iced_core::{self, image::Renderer as _, layout, Renderer};
use cosmic::iced_core::{event, Event, Shell};
use cosmic::widget::image::{FilterMethod, Handle};
use cosmic::widget::Widget;
use cosmic::{Apply, Element};
use image::{GenericImageView, Pixel};
use libmonotile::canvas::{Canvas, Tile};
use libmonotile::tileset::Tileset;

pub struct TileCanvas<'a> {
    current_tile: Tile,
    tileset: &'a Tileset,
    canvas: &'a Canvas,
    scale: u8,
}

pub fn tile_canvas<'a>(
    current_tile: Tile,
    tileset: &'a Tileset,
    canvas: &'a Canvas,
    scale: u8,
) -> TileCanvas<'a> {
    TileCanvas {
        current_tile,
        tileset,
        canvas,
        scale,
    }
}

impl<'a> TileCanvas<'a> {
    fn draw_dimensions(&self) -> Size<f32> {
        let scale = self.scale as f32;
        let (tile_width, tile_height) = self.tileset.tile_size;
        let (canvas_width, canvas_height) = self.canvas.size();
        let mut size = Size::new(
            tile_width as f32 * canvas_width as f32,
            tile_height as f32 * canvas_height as f32,
        );
        size.width *= scale;
        size.height *= scale;
        size
    }

    fn set_tile(&self, shell: &mut Shell<'_, Message>, position: Point) {
        let x = position.x as u32 / (self.scale as u32) / self.tileset.tile_size.0;
        let y = position.y as u32 / (self.scale as u32) / self.tileset.tile_size.1;
        let (width, height) = self.canvas.size();
        if x >= width || y >= height || self.canvas.get_tile(x, y) == self.current_tile {
            return;
        }
        shell.publish(Message::CanvasClicked { x, y });
    }
}

impl<'a> Widget<Message, cosmic::Theme, cosmic::iced::Renderer> for TileCanvas<'a> {
    fn tag(&self) -> iced_core::widget::tree::Tag {
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
        _tree: &mut iced_core::widget::Tree,
        _renderer: &cosmic::Renderer,
        _limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        layout::Node::new(self.draw_dimensions())
    }

    fn on_event(
        &mut self,
        tree: &mut tree::Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _renderer: &cosmic::iced::Renderer,
        _clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state: &mut State = tree.state.downcast_mut();
        match event {
            Event::Mouse(mouse_event) => {
                use iced_core::mouse;
                let position = cursor.position_in(layout.bounds());
                match mouse_event {
                    mouse::Event::CursorMoved { position: _ } => {
                        if state.dragging {
                            if let Some(position) = position {
                                self.set_tile(shell, position);
                            }
                            event::Status::Captured
                        } else {
                            event::Status::Ignored
                        }
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(position) = position {
                            state.dragging = true;
                            self.set_tile(shell, position);
                            event::Status::Captured
                        } else {
                            event::Status::Ignored
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.dragging {
                            state.dragging = false;
                            event::Status::Captured
                        } else {
                            event::Status::Ignored
                        }
                    }
                    _ => event::Status::Ignored,
                }
            }
            Event::Window(_, window_event) => match window_event {
                iced_core::window::Event::RedrawRequested(_) => {
                    state.update(self.tileset, self.current_tile, self.canvas);
                    event::Status::Ignored
                }
                _ => event::Status::Ignored,
            },
            _ => event::Status::Ignored,
        }
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut cosmic::Renderer,
        _theme: &cosmic::Theme,
        _style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state: &State = tree.state.downcast_ref();
        renderer.with_translation(layout.position() - Point::ORIGIN, |renderer| {
            let mut bounds = layout.bounds();
            bounds.x = 0.0;
            bounds.y = 0.0;
            renderer.draw(
                state.canvas_image.clone(),
                FilterMethod::Nearest,
                bounds,
                [0.0; 4],
            );
        });
        if let Some(mut position) = cursor.position_in(layout.bounds()) {
            let scale = self.scale as f32;
            let tile_width = self.tileset.tile_size.0 as f32 * scale;
            let tile_height = self.tileset.tile_size.1 as f32 * scale;
            position.x /= tile_width;
            position.y /= tile_height;
            position.x = position.x.floor();
            position.y = position.y.floor();
            position.x *= tile_width;
            position.y *= tile_height;
            position = position + (layout.position() - Point::ORIGIN);
            renderer.with_translation(position - Point::ORIGIN, |renderer| {
                let bounds = Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: tile_width,
                    height: tile_height,
                };
                renderer.draw(
                    state.tile_image.clone(),
                    FilterMethod::Nearest,
                    bounds,
                    [0.0; 4],
                );
            });
        }
    }
}

impl<'a> From<TileCanvas<'a>> for Element<'a, Message> {
    fn from(canvas: TileCanvas<'a>) -> Self {
        Self::new(canvas)
    }
}

pub struct State {
    canvas_image: Handle,
    tile_image: Handle,
    previous_tileset: Option<Tileset>,
    previous_tile: Option<Tile>,
    previous_canvas: Option<Canvas>,
    dragging: bool,
}

impl State {
    pub fn new() -> State {
        State {
            canvas_image: Handle::from_pixels(0, 0, []),
            tile_image: Handle::from_pixels(0, 0, []),
            previous_tileset: None,
            previous_tile: None,
            previous_canvas: None,
            dragging: false,
        }
    }

    pub fn update(&mut self, tileset: &Tileset, current_tile: Tile, canvas: &Canvas) {
        self.update_tile(tileset, current_tile);
        if self.previous_tileset.as_ref() == Some(tileset)
            && self.previous_canvas.as_ref() == Some(canvas)
        {
            return;
        }
        self.previous_tileset = Some(tileset.clone());
        self.previous_canvas = Some(canvas.clone());

        let mut canvas_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_pixel(
                canvas.width * tileset.tile_size.0,
                canvas.height * tileset.tile_size.1,
                image::Rgba([0, 0, 0, 255]),
            );
        for (x, y, tile) in canvas.tiles() {
            if let Some(tile_image) = tileset.tile(tile.index) {
                let xoff = x * tileset.tile_size.0;
                let yoff = y * tileset.tile_size.1;
                let mut foreground: image::Rgba<u8> = tile.fg.into();
                for (x, y, blend) in tile_image.pixels() {
                    foreground.0[3] = blend.0[0];
                    let mut color: image::Rgba<u8> = tile.bg.into();
                    color.blend(&foreground);
                    let x = x + xoff;
                    let y = y + yoff;
                    canvas_image.put_pixel(x, y, color);
                }
            }
        }
        self.canvas_image = Handle::from_pixels(
            canvas_image.width(),
            canvas_image.height(),
            canvas_image.into_raw(),
        );
    }

    fn update_tile(&mut self, tileset: &Tileset, current_tile: Tile) {
        if self.previous_tile == Some(current_tile) {
            return;
        }
        self.previous_tile = Some(current_tile);
        let tile_size = tileset.tile_size;
        let mut fg: image::Rgba<u8> = current_tile.fg.into();
        let bg = current_tile.bg.into();
        let mut tile_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_pixel(tile_size.0, tile_size.1, bg);
        for (pixel, (_, _, image::Luma([blend]))) in tile_image
            .pixels_mut()
            .zip(tileset.tile(current_tile.index).unwrap().pixels())
        {
            fg.0[3] = blend;
            pixel.blend(&fg);
        }
        self.tile_image = Handle::from_pixels(
            tile_image.width(),
            tile_image.height(),
            tile_image.into_raw(),
        );
    }
}
