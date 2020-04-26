use crate::screen_buffer::Color;

pub trait DrawApi {
  fn draw_apply(&mut self);
  fn draw_api(&mut self, x: usize, y: usize, color: Color);
  fn draw_item(
    &mut self,
    size: usize,
    x: usize,
    y: usize,
    color: Color
  );
}
