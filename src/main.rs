mod lifegame;

use lifegame::*;
use piston_window::*;

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
