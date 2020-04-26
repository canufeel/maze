use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::draw_api::DrawApi;

#[derive(Copy, Clone)]
pub enum Color {
  Black,
  White
}

impl Into<[u8; 4]> for Color {
  fn into(self) -> [u8; 4] {
    match self {
      Color::White => {
        [255, 255, 255, 255]
      },
      Color::Black => {
        [0, 0, 0, 255]
      }
    }
  }
}

pub struct ScreenBuffer {
  block_size: usize,
  columns: usize,
  rows: usize,
  draw_api: Box<dyn DrawApi>,
  buffer: Vec<Vec<Color>>
}

impl ScreenBuffer {
  pub fn new(
    block_size: usize,
    columns: usize,
    rows: usize,
    draw_api: Box<dyn DrawApi>
  ) -> Self {
    let mut inner_buf = Vec::new();
    for x in 0..(columns * block_size + 1) {
      inner_buf.push(Vec::new());
      for _ in 0..(rows * block_size + 1) {
        inner_buf[x].push(Color::White);
      }
    }
    let mut buf = ScreenBuffer {
      block_size,
      columns,
      rows,
      draw_api,
      buffer: inner_buf
    };
    buf.initial_fill();
    buf
  }

  fn fill_horizontal_line(&mut self, x0: usize, x1: usize, y: usize, color: Color) {
    let left;
    let right;
    if x0 < x1 {
      left = x0;
      right = x1;
    } else {
      left = x1;
      right = x0;
    }
    for x in left..right {
      self.buffer[x][y] = color;
    }
  }

  fn fill_vertical_line(&mut self, y0: usize, y1: usize, x: usize, color: Color) {
    let top;
    let bottom;
    if y0 < y1 {
      top = y0;
      bottom = y1;
    } else {
      top = y1;
      bottom = y0;
    }
    for y in top..bottom {
      self.buffer[x][y] = color;
    }
  }

  fn initial_fill(&mut self) {
    let color = Color::Black;
    for col in 0..self.columns {
      for row in 0..self.rows {
        self.fill_horizontal_line(col * self.block_size, (col + 1) * self.block_size, row * self.block_size, color);
        self.fill_horizontal_line(col * self.block_size, (col + 1) * self.block_size, (row + 1) * self.block_size, color);
        self.fill_vertical_line(row * self.block_size, (row + 1) * self.block_size, col * self.block_size, color);
        self.fill_vertical_line(row * self.block_size, (row + 1) * self.block_size, (col + 1) * self.block_size, color);
      }
    }
  }
  pub fn remove_wall(&mut self, a_idx: usize, b_idx: usize) {
    let color = Color::White;
    let left;
    let right;
    if a_idx < b_idx {
      left = a_idx;
      right = b_idx;
    } else {
      left = b_idx;
      right = a_idx;
    }
    let left_col = left % self.columns;
    let left_row = left / self.columns;
    let right_col = right % self.columns;
    let right_row = right / self.columns;
    match (left, right) {
      (l, r) if l + 1 == r => {
        self.fill_vertical_line(
          left_row * self.block_size,
          (left_row + 1) * self.block_size,
          right_col * self.block_size,
          color
        );
      },
      (l, r) if l + 1 != r => {
        self.fill_horizontal_line(
          left_col * self.block_size,
          (left_col + 1) * self.block_size,
          right_row * self.block_size,
          color
        );
      },
      (_, _) => unreachable!()
    }
  }

  pub fn draw(&mut self) {
    for x in 0..self.buffer.len() {
      for y in 0..self.buffer[x].len() {
        self.draw_api.draw_api(x, y, self.buffer[x][y]);
      }
    }
    self.draw_api.draw_apply();
  }
}

#[cfg(test)]
mod tests {
  use rand::Rng;
  use super::*;
  use crate::maze::Maze;

  struct ApiMock {}

  impl DrawApi for ApiMock {
    fn draw_apply(&mut self) {}
    fn draw_api(&mut self, x: usize, y: usize, color: Color) {}
  }

  #[test]
  fn can_construct_screen_buffer() {
    let draw_api = |_: usize, _: usize, _: Color| {};
    let draw_apply = || {};
    let buf = ScreenBuffer::new(
      5,
      5,
      5,
      Box::new(ApiMock {})
    );
  }

  #[test]
  fn can_fill_buffer() {
    let rnd = |x: usize, y: usize| -> usize {
      let mut rng = rand::thread_rng();
      rng.gen_range(x, y)
    };
    let w = 5;
    let h = 5;
    let maze = Maze::new(
      w,
      h,
      Box::new(rnd)
    );
    let draw_api = |_: usize, _: usize, _: Color| {};
    let draw_apply = || {};
    let draw_apply = || {};
    let mut buf = ScreenBuffer::new(
      5,
      w,
      h,
      Box::new(ApiMock {})
    );
    maze.feed_whitespace(&mut buf);
  }
}
