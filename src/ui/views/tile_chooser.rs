use vizia::prelude::*;
use vizia::vg;

use crate::ui::{canvas::Tile, State};

const SCALE: f32 = 2.0;

pub fn build(cx: &mut Context) {
    Binding::new(cx, State::tileset, |cx, tileset| {
        Binding::new(cx, State::current_tile, move |cx, tile| {
            let tileset = tileset.get(cx);
            TileChooser {
                current_tile: tile.get(cx),
            }
            .build(cx, |_| {})
            .width(Units::Pixels(
                (tileset.image().width() as f32 + 2.0) * SCALE,
            ))
            .height(Units::Pixels(
                (tileset.image().height() as f32 + 2.0) * SCALE,
            ))
            .left(Units::Stretch(1.0))
            .right(Units::Stretch(1.0));
        })
    });
}

struct TileChooser {
    current_tile: Tile,
}

impl View for TileChooser {
    fn element(&self) -> Option<&'static str> {
        Some("tilechooser")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}

    fn draw(&self, cx: &mut DrawContext, canvas: &mut vizia::view::Canvas) {
        canvas.save_with(|canvas| {
            let bounds = cx.bounds();

            //Skip widgets with no width or no height
            if bounds.w == 0.0 || bounds.h == 0.0 {
                return;
            }

            let mut path = vg::Path::new();
            path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
            canvas.fill_path(&mut path, vg::Paint::color(vg::Color::black()));

            canvas.translate(bounds.x, bounds.y);
            canvas.scale(SCALE, SCALE);
            canvas.translate(1.0, 1.0);

            let tileset = &cx.data::<State>().unwrap().tileset;
            let current_tile = self.current_tile;
            let tileset_id = tileset.id(canvas);
            let width = tileset.image().width() as f32;
            let height = tileset.image().height() as f32;

            let tileset_paint = vg::Paint::image_tint(
                tileset_id,
                0.0,
                0.0,
                width,
                height,
                0.0,
                current_tile.fg.into(),
            )
            .with_anti_alias(false);
            let mut path = vg::Path::new();
            path.rect(0.0, 0.0, width, height);

            canvas.fill_path(&mut path, vg::Paint::color(current_tile.bg.into()));
            canvas.fill_path(&mut path, tileset_paint);

            let mut path = vg::Path::new();
            path.rect(
                -0.5 + tileset.tile_position(current_tile.index).x,
                -0.5 + tileset.tile_position(current_tile.index).y,
                tileset.tile_size.x as f32 + 1.0,
                tileset.tile_size.y as f32 + 1.0,
            );
            canvas.stroke_path(&mut path, vg::Paint::color(vg::Color::rgb(255, 0, 0)));
            canvas.flush();
        });
    }
}
