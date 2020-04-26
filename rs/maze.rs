use crate::disjoint_set::DisjointSet;
use crate::screen_buffer::ScreenBuffer;
use alloc::borrow::Borrow;
use alloc::boxed::Box;
use alloc::collections::btree_set::BTreeSet;
use alloc::vec::Vec;
use core::ops::Range;
use alloc::collections::vec_deque::VecDeque;

pub struct Maze {
  ds: DisjointSet,
  w: usize,
  h: usize,
  temp_connected_els: BTreeSet<usize>,
  rng: Box<dyn Fn(usize, usize) -> usize>
}

impl Maze {
  pub fn new(w: usize, h: usize, rng: Box<dyn Fn(usize, usize) -> usize>) -> Self {
    let mut maze = Maze {
      ds: DisjointSet::new(w * h),
      w,
      h,
      temp_connected_els: BTreeSet::new(),
      rng
    };

    for _ in 0..maze.size() / 4 {
      let (a, b) = maze.find_wall_pair_and_el();
      maze.ds.union(a, b);
      maze.temp_connected_els.insert(a);
      maze.temp_connected_els.insert(b);
    }

    let full_rng = Range { start: 0, end: maze.size() };
    let mut not_included_idxes: VecDeque<usize> = full_rng
      .filter(|i| !maze.temp_connected_els.contains(i))
      .collect();

    while not_included_idxes.len() > 0 {
      let (a, b) = maze.find_wall_pair_from_leftovers(&mut not_included_idxes);
      maze.ds.union(a, b);
    }

    let mut iterations = 0;
    let max_iters = 10;
    while let Some(idx) = maze.ds.next_root_idx() {
      if let Some((a, b)) = maze.find_wall_pair(idx) {
        maze.ds.union(a, b);
      } else {
        iterations += 1;
        if iterations == max_iters {
          break;
        }
      }
    }

    while !maze.ds.is_single_set() {
      let (a, b) = maze.find_wall_pair_and_el();
      maze.ds.union(a, b);
    }

    maze
  }

  fn size(&self) -> usize {
    self.w * self.h
  }

  fn find_wall_pair_from_leftovers(&self, leftovers: &mut VecDeque<usize>) -> (usize, usize) {
    let rng: &dyn Fn(usize, usize) -> usize = self.rng.borrow();
    let el_idx = rng(0, leftovers.len());
    if let Some(el) = leftovers.remove(el_idx) {
      if let Some(pair) = self.find_wall_pair(el) {
        return pair;
      }
      unreachable!();
    }
    unreachable!();
  }

  fn find_wall_pair_and_el(&self) -> (usize, usize) {
    let rng: &dyn Fn(usize, usize) -> usize = self.rng.borrow();
    let el = rng(0, self.w * self.h);
    if let Some((a, b)) = self.find_wall_pair(el) {
      return (a, b);
    }
    self.find_wall_pair_and_el()
  }

  fn find_wall_pair(&self, el: usize) -> Option<(usize, usize)> {
    let rng: &dyn Fn(usize, usize) -> usize = self.rng.borrow();
    let wall_start = rng(0, 3);
    for i in 0..3 {
      let possible_wall = match (wall_start + i) % 4 {
        0 => el > 0 && el % self.w != 0 && self.ds.find(el) != self.ds.find(el - 1), // prev column
        1 => el < self.w * self.h && (el + 1) % self.w != 0 && self.ds.find(el) != self.ds.find(el + 1), // next column
        2 => el > self.w && self.ds.find(el) != self.ds.find(el - self.w), // prev row
        3 => el < self.w * self.h - self.w && self.ds.find(el) != self.ds.find(el + self.w), // next row
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
        return Some(walls);
      }
    }
    None
  }

  pub fn debug(&self) -> Vec<isize> {
    self.ds.debug()
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
      20,
      20,
      Box::new(rnd)
    );
  }
}
