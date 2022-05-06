mod point;
mod cubic_curve;
mod canvas2d;
mod hittable;
use point::Point;
use canvas2d::Canvas2d;

use crate::cubic_curve::CubicCurve;

fn main() {
	let p = Point(1.0, 2.0);
	println!("{}", p.0);

	let curve = CubicCurve (
		Point(0.0, 0.0),
		Point(0.0, 100.0),
		Point(100.0, 100.0),
		Point(100.0, 0.0),
	);

	let mut canvas = Canvas2d::new(100, 100);
	canvas.add(curve);
	canvas.render();
}
