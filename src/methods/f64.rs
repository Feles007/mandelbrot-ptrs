use crate::parameters::{Extents, Parameters};
use crate::screen_buffer::ScreenBuffer;
use crate::{get_color, mix};
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

			let iterations_result = mandelbrot_f64(scaled_x, scaled_y, parameters.iterations);

			let color = get_color(parameters.iterations, iterations_result);

			pixel.r = color[0];
			pixel.g = color[1];
			pixel.b = color[2];
		});
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
