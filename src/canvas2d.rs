use crate::drawable::Drawable;

pub struct Canvas2d {
  pub width: u32,
  pub height: u32,
  drawable_objects: Vec<Box<dyn Drawable>>,
}

impl Canvas2d {
  pub fn new(width: u32, height: u32) -> Canvas2d {
    Canvas2d {
      width,
      height,
      drawable_objects: Vec::new(),
    }
  }

  pub fn add(&mut self, drawable: Box<dyn Drawable>) {
    self.drawable_objects.push(drawable);
  }

  pub fn render(&self) {
    println!("{}", self.height);
    println!("{}", self.width);
    self.drawable_objects.iter().for_each(
      |drawable| drawable.draw(self)
    );
  }
}