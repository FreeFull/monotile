use crate::ui::canvas::Tile;
use std::cell::Cell;
use std::convert::TryInto;
use std::fmt;
use vizia::image;
use vizia::prelude::*;
use vizia::vg;

const DEFAULT_TILESET_IMAGE: &'static [u8] = include_bytes!("../../data/tiles.png");

#[derive(Clone, PartialEq)]
pub struct Tileset {
    pub image_data: image::DynamicImage,
    image_id: Cell<Option<vg::ImageId>>,
    pub tile_size: (u8, u8),
}

impl Tileset {
    pub fn id(&self, canvas: &mut vg::Canvas<impl vg::Renderer>) -> vg::ImageId {
        if self.image_id.get().is_none() {
            let image_src: vg::ImageSource = (&self.image_data).try_into().unwrap();
            self.image_id.set(Some(
                canvas
                    .create_image(
                        image_src,
                        vg::ImageFlags::REPEAT_X
                            | vg::ImageFlags::REPEAT_Y
                            | vg::ImageFlags::NEAREST,
                    )
                    .unwrap(),
            ));
        }
        // image_id is always Some here
        self.image_id.get().unwrap()
    }

    pub fn image(&self) -> &image::DynamicImage {
        &self.image_data
    }

    pub fn tile_position(&self, index: u8) -> (usize, usize) {
        let index = index as usize;
        let tiles_per_line = self.image_data.width() as usize / self.tile_size.0 as usize;
        let tile_x = index % tiles_per_line;
        let tile_y = index / tiles_per_line;
        if tile_y <= (self.image_data.height() as usize / self.tile_size.1 as usize) {
            (
                tile_x * self.tile_size.0 as usize,
                tile_y * self.tile_size.1 as usize,
            )
        } else {
            (0, 0)
        }
    }
}

impl Default for Tileset {
    fn default() -> Self {
        let image = image::load_from_memory(DEFAULT_TILESET_IMAGE).unwrap();
        Tileset {
            image_data: image,
            image_id: Cell::new(None),
            tile_size: (8, 8),
        }
    }
}

impl fmt::Debug for Tileset {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("<Tileset>")
    }
}

impl Data for Tileset {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}
