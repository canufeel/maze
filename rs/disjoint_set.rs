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

  pub fn get_parent(&self, x: usize) -> Option<usize> {
    match self.s[x] {
      y if y < 0 => None,
      y => Some(y as usize)
    }
  }

  pub fn find(&self, x: usize) -> isize {
    if self.s[x] < 0 {
      return x as isize;
    } else {
      return self.find(self.s[x] as usize);
    }
  }

  pub fn union(&mut self, root_one: usize, root_two: usize) {
    self.s[root_two] = root_one as isize;
  }

  pub fn is_single_set(&self) -> bool {
    let mut count = 0;
    for i in &self.s {
      if *i == -1 && count == 1 {
        return false;
      } else if *i == -1 {
        count += 1;
      }
    }
    return count == 1;
  }
}
