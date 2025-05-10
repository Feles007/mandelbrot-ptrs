use crate::parameters::{Extents, Parameters};
use crate::screen_buffer::ScreenBuffer;
use crate::{get_color, mix};
use malachite::base::num::basic::traits::{Two, Zero};
use malachite::Float;
use rayon::prelude::*;

pub fn run(screen_buffer: &mut ScreenBuffer, parameters: &Parameters, extents: &Extents) {
	let width = screen_buffer.width();
	let height = screen_buffer.height();

	screen_buffer
		.buffer
		.par_iter_mut()
		.enumerate()
		.for_each(|(index, pixel)| {
			let x = (index as u32) % width;
			let y = (index as u32) / width;

			let scaled_x = mix(extents.hmin, extents.hmax, (x as f64) / (width as f64));
			let scaled_y = mix(extents.vmin, extents.vmax, (y as f64) / (height as f64));

			let iterations_result =
				mandelbrot_precise(Float::from(scaled_x), Float::from(scaled_y), parameters.iterations);

			let color = get_color(parameters.iterations, iterations_result);

			pixel.r = color[0];
			pixel.g = color[1];
			pixel.b = color[2];
		});
}
fn mandelbrot_precise(cr: Float, ci: Float, iterations: u32) -> u32 {
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
