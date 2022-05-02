use crate::canvas2d::Canvas2d;

pub trait Drawable {
  fn draw(&self, canvas: &Canvas2d);
}