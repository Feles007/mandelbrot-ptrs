use malachite::base::num::basic::traits::{Two, Zero};
use malachite::Float;

pub fn mandelbrot_f32(cr: f32, ci: f32, iterations: u32) -> u32 {
	let mut x = 0.0;
	let mut y = 0.0;
	for i in 0..iterations {
		if x * x + y * y > 4.0 {
			return i;
		}
		let new_x = x * x - y * y + cr;
		y = 2.0 * x * y + ci;
		x = new_x;
	}
	iterations
}
pub fn mandelbrot_f64(cr: f64, ci: f64, iterations: u32) -> u32 {
	let mut x = 0.0;
	let mut y = 0.0;
	for i in 0..iterations {
		if x * x + y * y > 4.0 {
			return i;
		}
		let new_x = x * x - y * y + cr;
		y = 2.0 * x * y + ci;
		x = new_x;
	}
	iterations
}
pub fn mandelbrot_precise(cr: Float, ci: Float, iterations: u32) -> u32 {
	let mut x = Float::ZERO;
	let mut y = Float::ZERO;

	for i in 0..iterations {
		let x2 = &x * &x;
		let y2 = &y * &y;
		if &x2 + &y2 > 4 {
			return i;
		}
		let new_x = x2 - y2 + &cr;
		y = Float::TWO * x * y + &ci;
		x = new_x;
	}
	iterations
}
