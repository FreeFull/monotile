use crate::ui::state::{State, Tool};
use vizia::{prelude::*, vg};

const SCALE: f32 = 2.0;

pub fn build(cx: &mut Context) {
    ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
        Binding::new(cx, State::canvas, |cx, canvas| {
            Binding::new(cx, State::tileset, move |cx, tileset| {
                let canvas_size = canvas.map(|canvas| canvas.size()).get(cx);
                let tile_size = tileset.map(|tileset| tileset.tile_size).get(cx);
                DrawingArea {}
                    .build(cx, |cx| {})
                    .width(Units::Pixels(
                        canvas_size.0 as f32 * SCALE * tile_size.0 as f32,
                    ))
                    .height(Units::Pixels(
                        canvas_size.1 as f32 * SCALE * tile_size.1 as f32,
                    ))
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
            let state = cx.data::<State>().unwrap();
            let tile_size = state.tileset.tile_size;
            for (x, y, tile) in state.canvas.tiles() {
                let (tile_x, tile_y) = state.tileset.tile_position(tile.index);
                let mut path = vg::Path::new();
                path.rect(
                    x as f32 * tile_size.0 as f32 * SCALE,
                    y as f32 * tile_size.1 as f32 * SCALE,
                    tile_size.0 as f32 * SCALE,
                    tile_size.1 as f32 * SCALE,
                );
                canvas.fill_path(&mut path, vg::Paint::color(tile.bg.into()));
                let image_id = state.tileset.id(canvas);
                canvas.fill_path(
                    &mut path,
                    vg::Paint::image_tint(
                        image_id,
                        (x as f32 * tile_size.0 as f32 - tile_x as f32) * SCALE,
                        (y as f32 * tile_size.1 as f32 - tile_y as f32) * SCALE,
                        state.tileset.image().width() as f32 * SCALE,
                        state.tileset.image().height() as f32 * SCALE,
                        0.0,
                        tile.fg.into(),
                    ),
                )
            }
        })
    }
}
