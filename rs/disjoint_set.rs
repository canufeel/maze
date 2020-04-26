use alloc::vec;

pub struct DisjointSet {
  s: vec::Vec<isize>
}

impl DisjointSet {
  pub fn new(size: usize) -> Self {
    DisjointSet {
      s: vec![-1; size]
    }
  }

  pub fn debug(&self) -> vec::Vec<isize> {
    self.s.clone()
  }

  pub fn get_parent(&self, x: usize) -> Option<usize> {
    match self.s[x] {
      y if y < 0 => None,
      y => Some(y as usize)
    }
  }

  pub fn find(&self, x: usize) -> isize {
    match self.s[x] < 0 {
      true => x as isize,
      false => self.find(self.s[x] as usize)
    }
  }

  pub fn union(&mut self, root_one: usize, root_two: usize) {
    self.s[root_two] = root_one as isize;
  }

  pub fn next_root_idx(&self) -> Option<usize> {
    let mut count = 0;
    for (idx, i) in self.s.iter().enumerate() {
      if *i == -1 && count == 1 {
        return Some(idx);
      } else if *i == -1 {
        count += 1;
      }
    }
    if count == 0 {
      unreachable!();
    }
    None
  }

  pub fn is_single_set(&self) -> bool {
    match self.next_root_idx() {
      Some(x) => false,
      None => true
    }
  }
}
