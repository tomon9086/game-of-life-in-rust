use piston_window::*;
use rand::prelude::*;

struct Map {
  width: usize,
  height: usize,
  cells: Vec<Cell>,
}

#[derive(Copy, Clone)]
struct Cell {
  x: usize,
  y: usize,
  // color: types::Color,
  is_alive: bool,
}

impl Map {
  fn new(width: usize, height: usize) -> Map {
    let mut map = Map {
      width: width,
      height: height,
      cells: vec![],
    };
    map.cells = Self::generate_cells(&map, width, height, |_map, _cell, x, y| {
      return Cell {
        x: x,
        y: y,
        is_alive: false,
      };
    });
    return map;
  }
  fn generate_cells(
    &self,
    width: usize,
    height: usize,
    cb: fn(&Map, &Option<Cell>, usize, usize) -> Cell,
  ) -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    for i in 0..width {
      for j in 0..height {
        let cell: Option<Cell> = None;
        cells.push(cb(self, &cell, i, j));
      }
    }
    return cells;
  }

  fn next_generation(&self) -> Vec<Cell> {
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

    return new_cells;
  }
}

impl Cell {
  fn fill(&self, color: types::Color) {
    println!("fill!!!");
    // let window = self.map.window;
    // let x = self.x;
    // let y = self.y;
    // let width = self.width;
    // let height = self.height;
    // if let Some(event) = window.next() {
    //   window.draw_2d(&event, |context, graphics, _device| {
    //     rectangle(
    //       color,
    //       [x as f64, y as f64, width, height],
    //       context.transform,
    //       graphics,
    //     );
    //   });
    // }
  }
}

fn open_window(width: u32, height: u32, title: &'static str) -> PistonWindow {
  return WindowSettings::new(title, [width, height])
    .exit_on_esc(true)
    .build()
    .unwrap();
}

fn draw(window: &mut PistonWindow) {
  let window_size = window.size();
  // let (window_width, window_height) = (window_size.width, window_size.height);
  let (cell_width, cell_height) = (16, 16);
  let (width, height) = (4, 4);
  // let mut i = 0;
  // while let Some(event) = window.next() {
  //   window.draw_2d(&event, |context, graphics, _device| {
  //     clear([1.0; 4], graphics);
  //     // println!("{}", i);
  //     for i in 0..width {
  //       for j in 0..height {
  //         let x = (i as f64) * (cell_width as f64);
  //         let y = (j as f64) * (cell_height as f64);
  //         // println!("{}, {}", x, y)

  //         rectangle(
  //           [i as f32 / width as f32, 0.0, j as f32 / height as f32, 1.0],
  //           [x, y, cell_width as f64, cell_height as f64],
  //           context.transform,
  //           graphics,
  //         );
  //       }
  //     }
  //     // i += 1;
  //   });
  // }
}

fn main() {
  let (cell_width, cell_height) = (16, 16);
  let (width, height): (usize, usize) = (10, 10);
  let mut window = open_window(
    cell_width * (width as u32),
    cell_height * (height as u32),
    "Hello, Rust!",
  );
  let mut map = Map::new(width, height);

  for i in 0..width {
    for j in 0..height {
      map.cells[width * j + i].is_alive = random::<bool>();
    }
  }

  for g in 0..10 {
    println!("{}", g);
    println!("------------");
    for i in 0..width {
      let mut l = String::new();
      for j in 0..height {
        let mut c = " ";
        if map.cells[width * j + i].is_alive {
          c = "@";
        }
        l = format!("{}{}", l, c);
      }
      println!("|{}|", l);
    }
    println!("------------");
    println!("");
    map.cells = map.next_generation();
  }
}
