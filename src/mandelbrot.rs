use crate::parameters::Extents;
use malachite::base::num::basic::traits::{Two, Zero};
use malachite::Float;

#[derive(Copy, Clone)]
pub enum Precision {
	F32,
	F64,
	Arbitrary,
}
fn mix(x: f64, y: f64, a: f64) -> f64 {
	x * (1.0 - a) + y * a
}
fn get_color(iterations: u32, i: u32) -> [u8; 3] {
	let h = mix(0.0, 359.0, (i as f64) / (iterations as f64));

	let hp = h / 60.0;
	let z = 1.0 - (hp % 2.0 - 1.0).abs();

	let r;
	let g;
	let b;

	if hp < 1.0 {
		r = 1.0;
		g = z;
		b = 0.0;
	} else if hp < 2.0 {
		r = z;
		g = 1.0;
		b = 0.0;
	} else if hp < 3.0 {
		r = 0.0;
		g = 1.0;
		b = z;
	} else if hp < 4.0 {
		r = 0.0;
		g = z;
		b = 1.0;
	} else if hp < 5.0 {
		r = z;
		g = 0.0;
		b = 1.0;
	} else
	/*if (hp < 6.0)*/
	{
		r = 0.0;
		g = 0.0;
		b = z;
	} // r 1->0

	[(r * 255.999) as u8, (g * 255.999) as u8, (b * 255.999) as u8]
}
pub fn run(
	iterations: u32,
	x: u32,
	y: u32,
	width: u32,
	height: u32,
	extents: Extents,
	precision: Precision,
) -> [u8; 3] {
	let scaled_x = mix(extents.hmin, extents.hmax, (x as f64) / (width as f64));
	let scaled_y = mix(extents.vmin, extents.vmax, (y as f64) / (height as f64));
	let i = match precision {
		Precision::F32 => mandelbrot_f32(scaled_x as f32, scaled_y as f32, iterations),
		Precision::F64 => mandelbrot_f64(scaled_x, scaled_y, iterations),
		Precision::Arbitrary => mandelbrot_precise(Float::from(scaled_x), Float::from(scaled_y), iterations),
	};

	get_color(iterations, i)
}

fn mandelbrot_f32(cr: f32, ci: f32, iterations: u32) -> u32 {
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
fn mandelbrot_f64(cr: f64, ci: f64, iterations: u32) -> u32 {
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
fn mandelbrot_precise(cr: Float, ci: Float, iterations: u32) -> u32 {
	let mut x = Float::ZERO;
	let mut y = Float::ZERO;
	for i in 0..iterations {
		if x.clone() * x.clone() + y.clone() * y.clone() > 4.0 {
			return i;
		}
		let new_x = x.clone() * x.clone() - y.clone() * y.clone() + cr.clone();
		y = Float::TWO * x * y + ci.clone();
		x = new_x;
	}
	iterations
}
