use rand::prelude::*;

// Run parameters
const SPECIES: usize = 4000;

mod image;
mod field;
mod bio;

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
