use crate::*;

impl Map {
  pub fn generate_cells(
    &self,
    width: usize,
    height: usize,
    cb: fn(&Map, &Option<Cell>, usize, usize) -> Cell,
  ) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    for j in 0..height {
      for i in 0..width {
        let cell: Option<Cell> = None;
        cells.push(cb(self, &cell, i, j));
      }
    }
    return cells;
  }
}
