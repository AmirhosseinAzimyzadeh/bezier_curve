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

	let width = 200.0;
	let height = 200.0;

	let curve = CubicCurve (
		Point(0.0, 0.0),
		Point(0.0, width - 1.0),
		Point(height - 1.0, 0.0),
		Point(height - 1.0, width - 1.0),
	);

	let mut canvas = Canvas2d::new(width as u32, height as u32);
	canvas.add(curve);
	canvas.render();
}
