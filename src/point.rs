use std::{ops::{Add, Mul}};

use crate::hittable::Hittable;

#[derive(Debug)]
pub struct Point(pub f32, pub f32);

impl Hittable for Point {
  fn hit(&self, target_point: Point) -> bool {
    self.0 == target_point.0 && self.1 == target_point.1
  }
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point(self.0 + other.0, self.1 + other.1)
  }
}

impl Mul<f32> for Point {
  type Output = Point;

  fn mul(self, rhs: f32) -> Point {
    Point(
      self.0 * rhs,
      self.1 * rhs,
    )
  }
}

impl Clone for Point {
  fn clone(&self) -> Point {
    Point(self.0, self.1)
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    let epsilon = 10.0;
    (self.0 - other.0).abs() < epsilon && (self.1 - other.1).abs() < epsilon
  }
}

impl Point {
  // pub fn get_position_on_canvas(&self, width: f32, height: f32) -> (f32, f32) {
  //   let x = self.0;
  //   let y = self.1;
  //   let x_pos = (x / width) * 100.0;
  //   let y_pos = (y / height) * 100.0;
  //   (x_pos, y_pos)
  // }
}