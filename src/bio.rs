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