use rand::prelude::*;

// Run parameters
const X: usize = 256;
const Y: usize = 256;
const SPECIES: usize = 4000;

// Imaging options
const IMG_SCALE_X: usize = 2;
const IMG_SCALE_Y: usize = 2;

// Implementation constants
const FIELD_SIZE: usize = X * Y;

///////////////////////////////////////

struct Tile {
    occupied: bool,
}

fn main() {
    let mut field = [&Tile { occupied: false }; FIELD_SIZE];

    let mut rng = rand::thread_rng();

    randmomly_spread_cells_over_tiles(&mut field, &mut rng);
    display_field(&field);
    save_image_field(&field);
}

fn randmomly_spread_cells_over_tiles(field: &mut [&Tile; FIELD_SIZE], rng: &mut ThreadRng) {
    let mut indexes: Vec<usize> = (0..FIELD_SIZE).collect();
    let (indexes, _) = indexes.partial_shuffle(rng, SPECIES);
    let indexes = Vec::from(indexes);

    for c in indexes {
        field[c] =  &Tile{ occupied: true }
    }
}

fn display_field(field: &[&Tile; FIELD_SIZE]) {
    for y in 0..Y {
        for x in 0..X {
            print!(" {}", if field[y * X + x].occupied { '+' } else { 'O' });
        }
        println!("");
    }
}

fn save_image_field(field: &[&Tile; FIELD_SIZE]) {
    let mut imgbuf = image::ImageBuffer::new(
        (X * IMG_SCALE_X).try_into().unwrap(),
        (Y * IMG_SCALE_Y).try_into().unwrap(),
    );

    for (px, py, pixel) in imgbuf.enumerate_pixels_mut() {
        let x: usize = (px / IMG_SCALE_X as u32).try_into().unwrap();
        let y: usize = (py / IMG_SCALE_Y as u32).try_into().unwrap();
        let taken = field[y * X + x].occupied;
        let r = if taken { 0 } else { 250 } as u8;
        let g = if taken { 0 } else { 250 } as u8;
        let b = if taken { 0 } else { 250 } as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("field.png").unwrap();
}
