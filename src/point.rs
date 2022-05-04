use crate::drawable::Drawable;
use crate::canvas2d::Canvas2d;

pub struct Point(pub u32, pub u32);

impl Drawable for Point {
  fn draw(&self, canvas: &Canvas2d) {
    // check if point is inside canvas
    if self.0 > canvas.width && self.1 > canvas.height {
      return;
    }
    
  }
}