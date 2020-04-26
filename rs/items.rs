use crate::screen_buffer::{ScreenBuffer, Color};

pub struct Apple {
  loc: usize
}

pub struct Exit {
  loc: usize
}

pub trait Locatable {
  fn get_loc(&self) -> usize;
}

impl Exit {
  pub fn new(
    loc: usize,
    buf: &mut ScreenBuffer
  ) -> Self {
    let item = Exit {
      loc,
    };
    buf.draw_item(item.loc, Color::Blue);
    item
  }
}

impl Locatable for Exit {
  fn get_loc(&self) -> usize {
    self.loc
  }
}

impl Apple {
  pub fn new(
    loc: usize,
    buf: &mut ScreenBuffer
  ) -> Self {
    let item = Apple {
      loc,
    };
    buf.draw_item(item.loc, Color::Orange);
    item
  }
}

impl Locatable for Apple {
  fn get_loc(&self) -> usize {
    self.loc
  }
}
