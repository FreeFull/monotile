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
            .width(Units::Pixels(tileset.image().width() as f32 * SCALE))
            .height(Units::Pixels(tileset.image().height() as f32 * SCALE));
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

            let tileset = &cx.data::<State>().unwrap().tileset;
            let current_tile = cx.data::<State>().unwrap().current_tile;
            let id = tileset.id(canvas);

            let paint = vg::Paint::image_tint(
                id,
                bounds.x,
                bounds.y,
                bounds.w,
                bounds.h,
                0.0,
                current_tile.fg.into(),
            )
            .with_anti_alias(false);
            let mut path = vg::Path::new();
            path.rect(bounds.x, bounds.y, bounds.w, bounds.h);

            canvas.fill_path(&mut path, vg::Paint::color(current_tile.bg.into()));
            canvas.fill_path(&mut path, paint);
            canvas.flush();
        });
    }
}
