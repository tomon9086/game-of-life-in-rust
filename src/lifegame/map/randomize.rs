use crate::*;
use rand::prelude::*;

impl Map {
  pub fn randomize(&mut self) {
    let width = self.width;
    let height = self.height;
    for i in 0..width {
      for j in 0..height {
        self.cells[width * j + i].is_alive = random::<bool>();
      }
    }
  }
}
