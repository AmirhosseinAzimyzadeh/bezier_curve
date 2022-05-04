use crate::{point::Point, hittable::Hittable};

pub struct CubicCurve (
  pub Point,
  pub Point,
  pub Point,
  pub Point,
);


impl Hittable for CubicCurve {
  fn hit(&self, _target_point: Point) -> bool {
    return true;
  }
}