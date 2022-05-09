use std::{fs::File, io::Write};

use crate::{ point::Point, cubic_curve::CubicCurve };

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

    let mut image_content = String::from(
      format!("P2\n{} {}\n15\n", self.width, self.height)
    );

    let valid_points = self.curves.get(0).unwrap()
      .get_points(0.0, 1.0, 0.1);

    // iterating over pixels
    for pixel_number in 0..(self.width * self.height) {
      let x = (pixel_number % self.width) as f32;
      let y = (pixel_number / self.width) as f32;
      
      // search in valid points
      match valid_points.iter().find(|point| {**point == Point(x, y)}) {
          Some(_) => { image_content.push_str("15 15 15 ")  },
          None => { image_content.push_str("0 0 0 ") },
      };

      if pixel_number % self.width == 0 {
        image_content.push_str("\n");
      }
    }



    let mut f = File::create("output.ppm")
      .expect("Unable to create file");
    f.write(image_content.as_bytes())
      .expect("Unable to write to file");
    
  }
}
