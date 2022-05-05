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

impl CubicCurve {
  pub fn get_point_at(&self, t: f32) -> Point {
    let one_minus_t = 1.0 - t;
    let one_minus_t_2 = one_minus_t * one_minus_t;
    let one_minus_t_3 = one_minus_t_2 * one_minus_t;

    let new_point = self.0.clone() * one_minus_t_3 +
      self.1.clone() * 3.0 * one_minus_t_2 * t +
      self.2.clone() * 3.0 * one_minus_t * t * t +
      self.3.clone() * t * t * t;
    Point(new_point.0, new_point.1)
  }
}
