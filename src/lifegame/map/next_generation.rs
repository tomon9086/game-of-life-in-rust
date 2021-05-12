use crate::*;

impl Map {
  pub fn next_generation(&mut self) {
    let new_cells = Self::generate_cells(self, self.width, self.height, |map, cell, x, y| {
      /*
       * around
       * 0 1 2
       * 3 _ 4
       * 5 6 7
       */
      let w = map.width;
      let h = map.height;
      let c = cell.unwrap_or(map.cells[y * w + x]);
      let mut around = vec![false, false, false, false, false, false, false, false];

      if 0 < y {
        if 0 < x {
          around[0] = map.cells[(y - 1) * w + (x - 1)].is_alive;
        }
        around[1] = map.cells[(y - 1) * w + x].is_alive;
        if x < w - 1 {
          around[2] = map.cells[(y - 1) * w + (x + 1)].is_alive;
        }
      }
      if 0 < x {
        around[3] = map.cells[y * w + (x - 1)].is_alive;
      }
      if x < w - 1 {
        around[4] = map.cells[y * w + (x + 1)].is_alive;
      }
      if y < h - 1 {
        if 0 < x {
          around[5] = map.cells[(y + 1) * w + (x - 1)].is_alive;
        }
        around[6] = map.cells[(y + 1) * w + x].is_alive;
        if x < w - 1 {
          around[7] = map.cells[(y + 1) * w + (x + 1)].is_alive;
        }
      }

      let alive_cells_around = around.into_iter().fold(0, |p, c| {
        if c {
          return p + 1;
        } else {
          return p;
        }
      });

      let mut next_alive = false;
      if alive_cells_around == 3 {
        next_alive = true;
      }
      if c.is_alive && alive_cells_around == 2 {
        next_alive = true;
      }

      return Cell {
        x: x,
        y: y,
        is_alive: next_alive,
      };
    });

    self.cells = new_cells;
  }
}
