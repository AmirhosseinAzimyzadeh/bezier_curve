use crate::hittable::Hittable;

pub struct Point(pub u32, pub u32);

impl Hittable for Point {
  fn hit(&self, target_point: Point) -> bool {
    self.0 == target_point.0 && self.1 == target_point.1
  }
}