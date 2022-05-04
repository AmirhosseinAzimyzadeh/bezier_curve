mod point;
mod cubic_curve;
mod canvas2d;
mod hittable;
use point::Point;
use canvas2d::Canvas2d;

use crate::cubic_curve::CubicCurve;

fn main() {
	let p = Point(1, 2);
	println!("{}", p.0);

	let curve = CubicCurve (
		Point(0, 0),
		Point(1, 1),
		Point(2, 2),
		Point(3, 3),
	);

	let mut canvas = Canvas2d::new(1080, 1920);
	canvas.add(Box::new(curve));
	canvas.render();
}
