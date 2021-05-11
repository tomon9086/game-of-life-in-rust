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
    for j in 0..height {
      for i in 0..width {
        let cell: Option<Cell> = None;
        cells.push(cb(self, &cell, i, j));
      }
    }
    return cells;
  }

  fn randomize(&mut self) {
    let width = self.width;
    let height = self.height;
    for i in 0..width {
      for j in 0..height {
        self.cells[width * j + i].is_alive = random::<bool>();
      }
    }
  }

  fn next_generation(&mut self) {
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

  fn get_2d_cells(&self) -> Vec<Vec<Cell>> {
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

fn open_window(width: u32, height: u32, title: &'static str) -> PistonWindow {
  let mut window: PistonWindow = WindowSettings::new(title, [width, height])
    .exit_on_esc(true)
    .build()
    .unwrap();
  // window.set_lazy(true);
  window.events.set_ups(3);
  window.events.max_fps(5);
  return window;
}

fn convert_coord_to_rect(
  map: &Map,
  window_size: Size,
  x: usize,
  y: usize,
) -> (types::Color, types::Rectangle) {
  let width = map.width;
  let height = map.height;
  let dead_color = [0., 0., 0., 1.];
  let alive_color = [0., 1., 0., 1.];
  let frame_width = 4.0;
  let border_width = 2.0;
  let (window_width, window_height) = (window_size.width, window_size.height);
  let (cell_width, cell_height) = (
    (window_width - 2. * frame_width + border_width) / (width as f64) - border_width,
    (window_height - 2. * frame_width + border_width) / (height as f64) - border_width,
  );
  let cell = map.cells[width * y + x];
  let px = frame_width + (x as f64) * (border_width + cell_width);
  let py = frame_width + (y as f64) * (border_width + cell_height);
  let mut color = dead_color;
  if cell.is_alive {
    color = alive_color;
  }
  return (color, [px, py, cell_width, cell_height]);
}

fn main() {
  let mut window = open_window(240, 240, "Game of Life in Rust");

  let mut map = Map::new(10, 10);
  map.randomize();

  let window_size = window.size();
  while let Some(event) = window.next() {
    window.draw_2d(&event, |context, graphics, _device| {
      clear([0.; 4], graphics);
      for x in 0..map.width {
        for y in 0..map.height {
          let (color, rect) = convert_coord_to_rect(&map, window_size, x, y);
          rectangle(color, rect, context.transform, graphics);
        }
      }
    });
    if let Some(_) = event.update_args() {
      map.next_generation();
    }
    if let Some(_) = event.press_args() {
      map.next_generation();
    }
  }
}
