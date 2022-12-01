use crate::ui::{
    canvas,
    state::{State, Tool},
};
use vizia::{prelude::*, vg};

const SCALE: f32 = 2.0;

pub fn build(cx: &mut Context) {
    ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
        Binding::new(cx, State::canvas, |cx, canvas| {
            Binding::new(cx, State::tileset, move |cx, tileset| {
                let canvas_size = canvas.map(|canvas| canvas.size()).get(cx);
                let tile_size = tileset.map(|tileset| tileset.tile_size).get(cx);
                DrawingArea {}
                    .build(cx, |_cx| {})
                    .width(Units::Pixels(canvas_size.0 as f32 * SCALE * tile_size.x))
                    .height(Units::Pixels(canvas_size.1 as f32 * SCALE * tile_size.y))
                    .right(Units::Pixels(16.0))
                    .bottom(Units::Pixels(16.0));
            });
        });
    });
}

struct DrawingArea {}

impl View for DrawingArea {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        canvas.save_with(|canvas| {
            let bounds = cx.bounds();
            if bounds.w == 0.0 || bounds.h == 0.0 {
                return;
            }
            canvas.translate(bounds.x, bounds.y);
            canvas.scale(SCALE, SCALE);
            let state = cx.data::<State>().unwrap();
            let tile_size = state.tileset.tile_size;
            for (x, y, tile) in state.canvas.tiles() {
                let offset = state.tileset.tile_position(tile.index);
                let mut path = vg::Path::new();
                path.rect(
                    x as f32 * tile_size.x,
                    y as f32 * tile_size.y,
                    tile_size.x,
                    tile_size.y,
                );
                canvas.fill_path(&mut path, &vg::Paint::color(tile.bg.into()));
                let image_id = state.tileset.id(canvas);
                canvas.fill_path(
                    &mut path,
                    &vg::Paint::image_tint(
                        image_id,
                        x as f32 * tile_size.x - offset.x,
                        y as f32 * tile_size.y - offset.y,
                        state.tileset.image().width() as f32,
                        state.tileset.image().height() as f32,
                        0.0,
                        tile.fg.into(),
                    ),
                )
            }
        })
    }

    fn element(&self) -> Option<&'static str> {
        Some("drawingarea")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event: &WindowEvent, _| match *event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::PressDown { mouse: true }
            | WindowEvent::MouseMove(_, _) => {
                if cx.mouse.left.state == MouseButtonState::Pressed {
                    let entity = cx.current();
                    let bbox = cx.cache.get_bounds(entity);
                    let x = (cx.mouse.cursorx - bbox.x) / SCALE;
                    let y = (cx.mouse.cursory - bbox.y) / SCALE;
                    let state = cx.data::<State>().unwrap();
                    let tile_size = state.tileset.tile_size;
                    let x = (x / tile_size.x).floor() as usize;
                    let y = (y / tile_size.y).floor() as usize;
                    cx.emit(match state.current_tool {
                        Tool::Draw => canvas::Action::SetTile {
                            x,
                            y,
                            tile: state.current_tile,
                        },
                        Tool::FloodFill => canvas::Action::FloodFill {
                            x,
                            y,
                            tile: state.current_tile,
                        },
                    });
                }
            }
            _ => {}
        });
    }
}
