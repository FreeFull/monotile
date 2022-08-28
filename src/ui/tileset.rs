use emath::*;
use image::Pixel;
use std::cell::Cell;
use std::convert::TryInto;
use std::fmt;
use vizia::prelude::*;
use vizia::vg;

const DEFAULT_TILESET_IMAGE: &'static [u8] = include_bytes!("../../data/tiles.png");

#[derive(Clone, PartialEq)]
pub struct Tileset {
    pub image_data: image::DynamicImage,
    image_id: Cell<Option<vg::ImageId>>,
    pub width: u32,
    pub height: u32,
    pub tile_size: Vec2,
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

    pub fn tile_position(&self, index: u32) -> Pos2 {
        let index = index as usize;
        let tile_x = index % (self.width as usize);
        let tile_y = index / (self.width as usize);
        if tile_y <= self.height as usize {
            Pos2 {
                x: tile_x as f32 * self.tile_size.x,
                y: tile_y as f32 * self.tile_size.y,
            }
        } else {
            Pos2 { x: 0.0, y: 0.0 }
        }
    }

    pub fn index_from_position(&self, pos: Pos2) -> u32 {
        let pos = pos.to_vec2() / self.tile_size;
        let x = (pos.x as u32).clamp(0, self.width - 1);
        let y = (pos.y as u32).clamp(0, self.height - 1);
        x + y * self.width
    }
}

impl Default for Tileset {
    fn default() -> Self {
        let mut image = image::load_from_memory(DEFAULT_TILESET_IMAGE)
            .unwrap()
            .into_rgba8();
        for pixel in image.pixels_mut() {
            // Convert into a transparent mask.
            pixel.0 = [
                255,
                255,
                255,
                // Greyscale value, with premultiplied alpha.
                ((pixel.to_luma().0[0] as u16 * pixel.0[3] as u16) >> 8) as u8,
            ];
        }
        let image = image.into();

        Tileset {
            image_data: image,
            image_id: Cell::new(None),
            width: 16,
            height: 16,
            tile_size: Vec2 { x: 8.0, y: 8.0 },
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
