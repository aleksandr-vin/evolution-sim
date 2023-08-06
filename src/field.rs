use std::fmt::{Display, Formatter};
use std::fmt;
use image::ImageResult;

use crate::image::{CreateImage, IMG_SCALE_X, IMG_SCALE_Y};

const X: usize = 256;
const Y: usize = 256;
const FIELD_SIZE: usize = X * Y;

pub struct Tile {
    pub occupied: bool,
}
pub struct Field {
      pub tiles: Vec<Tile>,
}

impl Field {
    pub fn new() -> Field {
        let tiles = std::iter::repeat_with(|| Tile { occupied: false } )
        .take(FIELD_SIZE)
        .collect::<Vec<_>>();

        Field { tiles: tiles }
    }

    pub fn at(&self, lat: usize, lon: usize) -> &Tile {
        &self.tiles[lat * self.lon() + lon]
    }

    pub fn size(&self) -> usize {
        FIELD_SIZE
    }

    pub fn lat(&self) -> usize {
        Y
    }

    pub fn lon(&self) -> usize {
        X
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for lat in 0..self.lat() {
            for lon in 0..self.lon() {
                write!(f, " {}", if self.at(lat,lon).occupied { '+' } else { 'O' })?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl CreateImage for Field {
    fn save_image(&self, path: &str) -> ImageResult<()> {
        let mut imgbuf = image::ImageBuffer::new(
            (X * IMG_SCALE_X).try_into().unwrap(),
            (Y * IMG_SCALE_Y).try_into().unwrap(),
        );
    
        for (px, py, pixel) in imgbuf.enumerate_pixels_mut() {
            let x: usize = (px / IMG_SCALE_X as u32).try_into().unwrap();
            let y: usize = (py / IMG_SCALE_Y as u32).try_into().unwrap();
            let taken = self.at(y, x).occupied;
            let r = if taken { 0 } else { 250 } as u8;
            let g = if taken { 0 } else { 250 } as u8;
            let b = if taken { 0 } else { 250 } as u8;
            *pixel = image::Rgb([r, g, b]);
        }
    
        imgbuf.save(path)
    }
}