use rand::prelude::*;

struct Cell {
    taken: bool,
}

const X: usize = 7;
const Y: usize = 5;
const SPECIES: usize = 4;

const FIELD_SIZE: usize = X * Y;

fn main() {
    let mut field = [&Cell { taken: false }; FIELD_SIZE];

    let mut rng = rand::thread_rng();

    take_cells(&mut field, &mut rng);
    display_field(&field);
}

fn take_cells(field: &mut [&Cell; FIELD_SIZE], rng: &mut ThreadRng) {
    let mut taken_cells: Vec<usize> = (0..FIELD_SIZE).collect();
    let (taken_cells, _) = taken_cells.partial_shuffle(rng, SPECIES);
    let taken_cells = Vec::from(taken_cells);

    for c in taken_cells {
        field[c] = &Cell {
            taken: true,
        }
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

