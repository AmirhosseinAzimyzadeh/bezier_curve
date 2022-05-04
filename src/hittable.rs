use crate::point::Point;

pub trait Hittable {
  fn hit (&self, target_point: Point) -> bool;
}