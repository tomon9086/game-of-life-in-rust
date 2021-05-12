use crate::*;

impl Map {
  pub fn new(width: usize, height: usize) -> Map {
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
}
