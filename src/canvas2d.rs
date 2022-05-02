pub struct Canvas2d {
  pub width: u32,
  pub height: u32,
}

impl Canvas2d {
  pub fn new(width: u32, height: u32) -> Canvas2d {
    Canvas2d {
      width,
      height,
    }
  }

  pub fn render(&self) {
    println!("{}", self.height);
    println!("{}", self.width);
  }
}