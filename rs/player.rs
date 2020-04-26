use crate::maze::Maze;
use crate::screen_buffer::{ScreenBuffer, Color};

pub enum MoveEventKey {
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  Other
}

impl From<u32> for MoveEventKey {
  fn from(val: u32) -> Self {
    match val {
      38 => MoveEventKey::ArrowUp,
      37 => MoveEventKey::ArrowLeft,
      39 => MoveEventKey::ArrowRight,
      40 => MoveEventKey::ArrowDown,
      _ => MoveEventKey::Other
    }
  }
}

pub struct Player {
  loc: usize,
  width: usize,
  height: usize
}

impl Player {
  pub fn new(loc: usize, width: usize, height: usize, buf: &mut ScreenBuffer) -> Self {
    let pl = Player {
      loc,
      width,
      height
    };
    buf.draw_player(pl.loc, Color::Green);
    pl
  }

  fn size(&self) -> usize {
    self.width * self.height
  }

  fn can_move(&self, next_loc: usize, maze: &Maze) -> bool {
    maze.has_no_wall(self.loc, next_loc)
  }

  pub fn on_move(&mut self, move_evt: MoveEventKey, maze: &Maze, buf: &mut ScreenBuffer) {
    let next_loc = match move_evt {
      MoveEventKey::ArrowDown if self.loc + self.width < self.size() => self.loc + self.width,
      MoveEventKey::ArrowUp if self.loc > self.width => self.loc - self.width,
      MoveEventKey::ArrowRight if self.loc != self.size() && (self.loc + 1) % self.width != 0 => self.loc + 1,
      MoveEventKey::ArrowLeft if self.loc != 0 && (self.loc - 1) % self.width != (self.width - 1) => self.loc - 1,
      _ => self.loc
    };
    if self.loc != next_loc {
      self.move_to_loc(next_loc, maze, buf);
    }
  }

  fn move_to_loc(&mut self, next_loc: usize, maze: &Maze, buf: &mut ScreenBuffer) {
    if self.can_move(next_loc, maze) {
      buf.draw_player(self.loc, Color::White);
      self.loc = next_loc;
      buf.draw_player(self.loc, Color::Green);
    }
  }
}
