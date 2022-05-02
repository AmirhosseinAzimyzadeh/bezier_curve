mod point;
mod cubic_curve;
mod canvas2d;
mod drawable;
use point::Point;
use canvas2d::Canvas2d;

use crate::cubic_curve::CubicCurve;

fn main() {
	let p = Point(1.0, 2.0, 3.0);
	println!("{}", p.0);

	let curve = CubicCurve (
		Point(0.0, 0.0, 0.0),
		Point(1.0, 1.0, 1.0),
		Point(2.0, 2.0, 2.0),
		Point(3.0, 3.0, 3.0),
	);

	let mut canvas = Canvas2d::new(1080, 1920);
	canvas.add(Box::new(curve));
	canvas.render();
}
