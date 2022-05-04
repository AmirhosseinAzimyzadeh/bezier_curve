use crate::{hittable::Hittable, point::Point};

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

  pub fn add(&mut self, hittable: Box<dyn Hittable>) {
    self.hittable_objects.push(hittable);
  }

  pub fn render(&self) {
    println!("{}", self.height);
    println!("{}", self.width);

    for i in 0..self.width {
      for j in 0..self.height {
        self.hittable_objects.iter()
          .for_each(| hittable | {
            if hittable.hit(Point(i, j)) {
              todo!();
            }
        });
      }
    }
    // self.hittable.iter().for_each(
    //   |drawable| drawable.draw(self)
    // );
  }
}