use crate::canvas2d::Canvas2d;

pub trait Drawable {
  fn draw(&self, canvas: &mut Canvas2d);
}