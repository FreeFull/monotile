use cosmic::iced_core::Hasher;
use image::{GenericImageView, ImageBuffer, Luma, SubImage};
use std::fmt;
use std::hash::{Hash, Hasher as _};
use std::sync::Arc;

const DEFAULT_TILESET_IMAGE: &'static [u8] = include_bytes!("../data/tiles.png");

#[derive(Clone, PartialEq, Eq)]
pub struct Tileset {
    pub id: u64,
    pub image: Arc<ImageBuffer<Luma<u8>, Vec<u8>>>,
    pub tile_size: (u32, u32),
}

impl Tileset {
    pub fn tile(&self, index: u32) -> Option<SubImage<&ImageBuffer<Luma<u8>, Vec<u8>>>> {
        let height = self.image.height() / self.tile_size.1;
        let (tile_x, tile_y) = self.tile_position(index);
        if tile_y < height {
            Some(self.image.view(
                tile_x * self.tile_size.0,
                tile_y * self.tile_size.1,
                self.tile_size.0,
                self.tile_size.1,
            ))
        } else {
            None
        }
    }

    pub fn tile_position(&self, index: u32) -> (u32, u32) {
        let width = self.image.width() / self.tile_size.0;
        let tile_x = index % width;
        let tile_y = index / width;
        (tile_x, tile_y)
    }

    pub fn index_from_pixel_position(&self, pos: (u32, u32)) -> u32 {
        let pos = (pos.0 / self.tile_size.0, pos.1 / self.tile_size.1);
        let x = pos.0.clamp(0, self.width() - 1);
        let y = pos.1.clamp(0, self.height() - 1);
        x + y * self.width()
    }

    pub fn width(&self) -> u32 {
        self.image.width() / self.tile_size.0
    }

    pub fn height(&self) -> u32 {
        self.image.height() / self.tile_size.1
    }
}

impl Default for Tileset {
    fn default() -> Self {
        let image = image::load_from_memory(DEFAULT_TILESET_IMAGE).unwrap();
        let data = image.into_luma8();
        let mut hasher = Hasher::default();
        data.hash(&mut hasher);

        Tileset {
            id: hasher.finish(),
            image: Arc::new(data),
            tile_size: (8, 8),
        }
    }
}

impl fmt::Debug for Tileset {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("<Tileset>")
    }
}
