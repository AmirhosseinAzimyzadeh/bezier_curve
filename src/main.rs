mod point;
mod cubic_curve;
mod canvas2d;
mod drawable;
use point::Point;
use canvas2d::Canvas2d;

fn main() {
	let p = Point(1.0, 2.0, 3.0);
	println!("{}", p.0);

	let canvas = Canvas2d::new(1080, 1920);
}
