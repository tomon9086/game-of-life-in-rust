use crate::*;

impl Map {
  pub fn get_2d_cells(&self) -> Vec<Vec<Cell>> {
    let width = self.width;
    let height = self.height;
    let mut cells: Vec<Vec<Cell>> = vec![];
    for j in 0..height {
      cells.push(vec![]);
      for i in 0..width {
        cells[j].push(self.cells[width * j + i]);
      }
    }
    return cells;
  }
}
