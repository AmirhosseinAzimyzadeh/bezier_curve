use std::ops::{Add, Mul};

use crate::hittable::Hittable;

pub struct Point(pub u32, pub u32);

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

impl Mul<u32> for Point {
  type Output = Point;

  fn mul(self, rhs: u32) -> Point {
    Point(self.0 * rhs, self.1 * rhs)
  }
}
