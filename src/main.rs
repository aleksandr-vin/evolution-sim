use rand::prelude::*;

// Run parameters
const X: usize = 256;
const Y: usize = 256;
const SPECIES: usize = 10000;

// Imaging options
const IMG_SCALE_X: usize = 5;
const IMG_SCALE_Y: usize = 5;

// Implementation constants
const FIELD_SIZE: usize = X * Y;

///////////////////////////////////////

struct Cell {
    taken: bool,
}

fn main() {
    let mut field = [&Cell { taken: false }; FIELD_SIZE];

    let mut rng = rand::thread_rng();

    take_cells(&mut field, &mut rng);
    display_field(&field);
    save_image_field(&field);
}

fn take_cells(field: &mut [&Cell; FIELD_SIZE], rng: &mut ThreadRng) {
    let mut taken_cells: Vec<usize> = (0..FIELD_SIZE).collect();
    let (taken_cells, _) = taken_cells.partial_shuffle(rng, SPECIES);
    let taken_cells = Vec::from(taken_cells);

    for c in taken_cells {
        field[c] = &Cell { taken: true }
    }
}

fn display_field(field: &[&Cell; FIELD_SIZE]) {
    for y in 0..Y {
        for x in 0..X {
            print!(" {}", if field[y * X + x].taken { '+' } else { 'O' });
        }
        println!("");
    }
}

fn save_image_field(field: &[&Cell; FIELD_SIZE]) {
    let mut imgbuf = image::ImageBuffer::new(
        (X * IMG_SCALE_X).try_into().unwrap(),
        (Y * IMG_SCALE_Y).try_into().unwrap(),
    );

    for (px, py, pixel) in imgbuf.enumerate_pixels_mut() {
        let x: usize = (px / IMG_SCALE_X as u32).try_into().unwrap();
        let y: usize = (py / IMG_SCALE_Y as u32).try_into().unwrap();
        let taken = field[y * X + x].taken;
        let r = if taken { 0 } else { 250 } as u8;
        let g = if taken { 0 } else { 250 } as u8;
        let b = if taken { 0 } else { 250 } as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("field.png").unwrap();
}
