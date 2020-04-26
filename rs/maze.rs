use crate::disjoint_set::DisjointSet;
use crate::screen_buffer::ScreenBuffer;
use alloc::borrow::Borrow;
use alloc::boxed::Box;

pub struct Maze {
  ds: DisjointSet,
  w: usize,
  h: usize,
  rng: Box<dyn Fn(usize, usize) -> usize>
}

impl Maze {
  pub fn new(w: usize, h: usize, rng: Box<dyn Fn(usize, usize) -> usize>) -> Self {
    let mut maze = Maze {
      ds: DisjointSet::new(w * h),
      w,
      h,
      rng
    };

    while !maze.ds.is_single_set() {
      let (a, b) = maze.find_wall_pair();
      maze.ds.union(a, b);
    }

    maze
  }

  fn find_wall_pair(&self) -> (usize, usize) {
    let rng: &dyn Fn(usize, usize) -> usize = self.rng.borrow();
    let el = rng(0, self.w * self.h);
    let wall_start = rng(0, 3);
    for i in 0..3 {
      let possible_wall = match (wall_start + i) % 4 {
        0 => el > 0 && (el - 1) % self.h != 0 && self.ds.find(el) != self.ds.find(el - 1),
        1 => el < self.w * self.h && (el + 1) % self.h != 0 && self.ds.find(el) != self.ds.find(el + 1),
        2 => el > self.w && self.ds.find(el) != self.ds.find(el - self.w),
        3 => el < self.w * self.h - self.w && self.ds.find(el) != self.ds.find(el + self.w),
        _ => unreachable!(),
      };
      if possible_wall {
        let walls = match (wall_start + i) % 4 {
          0 => (el, (el - 1)),
          1 => (el, (el + 1)),
          2 => (el, el - self.w),
          3 => (el, el + self.w),
          _ => unreachable!(),
        };
        return walls;
      }
    }
    self.find_wall_pair()
  }

  pub fn feed_whitespace(&self, buf: &mut ScreenBuffer) {
    for i in 0..(self.w * self.h) {
      if let Some(adjancent) = self.ds.get_parent(i) {
        buf.remove_wall(i, adjancent);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use rand::Rng;
  use super::*;

  #[test]
  fn can_construct_maze() {
    let rnd = |x: usize, y: usize| -> usize {
      let mut rng = rand::thread_rng();
      rng.gen_range(x, y)
    };
    let maze = Maze::new(
      5,
      5,
      Box::new(rnd)
    );
  }
}
