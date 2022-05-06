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

    let mut hit_count = 0;
    let mut number_of_valid_points = 0;

    self.curves.iter().for_each(|curve| {
      let valid_points = curve.get_points(
        0.0,
        1.0,
        0.001,
      );

      number_of_valid_points = valid_points.len();

      for j in 0..self.height {
        for i in 0..self.width {
          // println!("{}, {}", i, j);
          // check if is key point
          if curve.0 == Point(j as f32, i as f32) {
            println!("point 0")
          }

          if curve.1 == Point(j as f32, i as f32) {
            println!("point 1")
          }

          if curve.2 == Point(j as f32, i as f32) {
            println!("point 2")
          }

          if curve.3 == Point(j as f32, i as f32) {
            println!("point 3")
          }

          valid_points.iter().for_each(|point| {
            if point.clone() == Point(j as f32, i as f32) {
              hit_count += 1;
              // color
              image_content.push_str(&format!("15 15 15\t"));
              // println!("inside !! {}, {}", point.0, point.1);
              // println!("next !! {}, {}", j, i);
            } else {
              image_content.push_str(&format!("0 0 0\t"));
            }
          });
        }
        image_content.push_str("\n");
      }
    });

    println!("hit_count: {}", hit_count);
    println!("number of valid points {}", number_of_valid_points);
    let mut f = File::create("output.ppm")
      .expect("Unable to create file");
    f.write(image_content.as_bytes())
      .expect("Unable to write to file");
    
  }
}
