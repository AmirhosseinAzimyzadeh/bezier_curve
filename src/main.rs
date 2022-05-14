mod point;
mod cubic_curve;
mod canvas2d;
mod hittable;
use point::Point;
use canvas2d::Canvas2d;

use crate::cubic_curve::CubicCurve;

fn main() {
	// canvas size
	let width = 200.0;
	let height = 200.0;

	// define cubic curve
	let curve = CubicCurve (
		Point(0.0, 0.0),
		Point(0.0, width - 1.0),
		Point(height - 1.0, 0.0),
		Point(height - 1.0, width - 1.0),
	);

	// create canvas
	let mut canvas = Canvas2d::new(width as u32, height as u32);
	
	// add curve to canvas
	canvas.add(curve);

	canvas.render(); // output.ppm
}
