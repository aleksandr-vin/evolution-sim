use rand::prelude::*;

// Run parameters
const SPECIES: usize = 4000;

///////////////////////////////////////

mod image {
    use image::ImageResult;

    // Imaging options
    pub const IMG_SCALE_X: usize = 2;
    pub const IMG_SCALE_Y: usize = 2;


    pub trait CreateImage {
        fn save_image(&self, file_name: &str) -> ImageResult<()>;
    }

}


mod field {
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
    
}


mod bio {
    use std::str::Bytes;
    
    #[derive(Debug)]
    pub struct Cell<'a> {
        id: String,
        genom: Bytes<'a>,
    }
    pub struct Biocoenosis<'a> {
        pub cells: Vec<Cell<'a>>,
    }

    impl<'a> Biocoenosis<'a> {
        pub fn new(size: usize) -> Biocoenosis<'a> {
            let mut cells: Vec<Cell> = Vec::with_capacity(size);
            for i in 1..size {
                let cell = Cell {
                    id: format!("c-{}", i),
                    genom: "xxx".bytes()
                };
                eprintln!("New cell {:?}", cell);
                cells.push(cell);
            }
            Biocoenosis { cells: cells }
        }
    }
}

use field::Field;
use bio::{Cell, Biocoenosis};
use crate::image::CreateImage;

impl Field {
    fn randmomly_spread_cells(&mut self, rng: &mut ThreadRng, cells: Vec<Cell>) {
        let mut indexes: Vec<usize> = (0..self.size()).collect();
        let (indexes, _) = indexes.partial_shuffle(rng, cells.len());
        let indexes = Vec::from(indexes);

        for c in indexes {
            self.tiles[c].occupied = true;
        }
    }
}

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut field = Field::new();
    let bio = Biocoenosis::new(SPECIES);
    let mut rng = rand::thread_rng();

    field.randmomly_spread_cells(&mut rng, bio.cells); 
    print!("{}", &field);
    match field.save_image("field.png") {
        Err(error) => panic!("Problem saving the image: {:?}", error),
        Ok(r) => Ok(r),
    }
}
