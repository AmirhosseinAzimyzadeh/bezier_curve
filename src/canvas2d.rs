use crate::{drawable::Drawable, hittable::Hittable};

pub struct Canvas2d {
  pub width: u32,
  pub height: u32,
  hittable_objects: Vec<Box<dyn Hittable>>,
}

impl Canvas2d {
  pub fn new(width: u32, height: u32) -> Canvas2d {
    Canvas2d {
      width,
      height,
      hittable_objects: Vec::new(),
    }
  }

  pub fn add(&mut self, drawable: Box<dyn Hittable>) {
    self.hittable_objects.push(drawable);
  }

  pub fn render(&self) {
    println!("{}", self.height);
    println!("{}", self.width);
    // self.drawable_objects.iter().for_each(
    //   |drawable| drawable.draw(self)
    // );
  }
}