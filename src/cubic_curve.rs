use crate::{point::Point, drawable::Drawable, canvas2d::Canvas2d};

pub struct CubicCurve (
  pub Point,
  pub Point,
  pub Point,
  pub Point,
);

impl Drawable for CubicCurve {
  fn draw(&self, canvas: &Canvas2d) {
    println!("{}", canvas.height);
    unimplemented!();
  }
}
