mod point;
mod cubic_curve;
use point::Point;

fn main() {
	let p = Point(1.0, 2.0, 3.0);
	println!("{}", p.0);
}
