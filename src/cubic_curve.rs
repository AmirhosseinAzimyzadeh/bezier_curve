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
    println!("{}, {}", new_point.0, new_point.1);
    Point(new_point.0, new_point.1)
  }

  pub fn get_points(&self, from: f32, to: f32, step: f32) -> Vec<Point> {
    let mut points = Vec::new();
    let mut t = from;
    while t < to {
      points.push(self.get_point_at(t));
      t += step;
    }
    points
  }
}
