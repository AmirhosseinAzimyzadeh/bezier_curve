use std::{fs::File, io::Write};

use crate::{ point::Point, cubic_curve::{CubicCurve, self} };

pub struct Canvas2d {
  pub width: u32,
  pub height: u32,
  curves: Vec<CubicCurve>,
}

impl Canvas2d {
  pub fn new(width: u32, height: u32) -> Canvas2d {
    Canvas2d {
      width,
      height,
      curves: Vec::new(),
    }
  }

  pub fn add(&mut self, curve: CubicCurve) {
    self.curves.push(curve);
  }

  pub fn render(&self) {
    println!("{}", self.height);
    println!("{}", self.width);

    // create ppm header
    let mut image_content = String::from(
      format!("P2\n{} {}\n255\n", self.width, self.height)
    );

    let valid_points = self.curves.get(0).unwrap()
      .get_points(0.0, 1.0, 0.1);

    // iterating over pixels
    for pixel_number in 0..(self.width * self.height) {
      let x = (pixel_number % self.width) as f32;
      let y = (pixel_number / self.width) as f32;

      let current_point = Point(x, y);
      
      if current_point == self.curves.get(0).unwrap().0 {
        image_content.push_str("255 0 0 ");
        break;
      }
      
      if current_point == self.curves.get(0).unwrap().1 {
        image_content.push_str("255 0 0 ");
        break;
      }

      if current_point == self.curves.get(0).unwrap().2 {
        image_content.push_str("255 0 0 ");
        break;
      }

      if current_point == self.curves.get(0).unwrap().3 {
        image_content.push_str("255 0 0 ");
        break;
      }
      // search in valid points
      match valid_points.iter().find(|point| {**point == current_point}) {
          Some(_) => { image_content.push_str("0 0 0 ")  },
          None => { image_content.push_str("255 255 255 ") },
      };

      // image_content.push_str("255 255 255 ");

      if (pixel_number + 1) % self.width == 0 {
        image_content.push_str("\n");
      }
    }



    let mut f = File::create("output.ppm")
      .expect("Unable to create file");
    f.write(image_content.as_bytes())
      .expect("Unable to write to file");
    
  }
}
