mod mandelbrot;
mod parameters;
mod screen_buffer;

use crate::mandelbrot::Precision;
use crate::parameters::Parameters;
use crate::screen_buffer::ScreenBuffer;
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;
use malachite::base::num::conversion::traits::RoundingFrom;
use malachite::base::rounding_modes::RoundingMode;
use malachite::Float;
use rayon::prelude::*;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn window_conf() -> Conf {
	Conf {
		window_title: "Window name".to_owned(),
		platform: Platform {
			swap_interval: Some(0),
			..Default::default()
		},
		..Default::default()
	}
}
#[macroquad::main(window_conf)]
async fn main() {
	let mut parameters = Parameters {
		center_x: Float::from(-0.5),
		center_y: Float::from(0.0),
		scale: Float::from(1.0),
		iterations: 1024,
		precision: Precision::F32,
	};

	let float_precision = 1024;

	parameters.center_x.set_prec(float_precision);
	parameters.center_y.set_prec(float_precision);
	parameters.scale.set_prec(float_precision);

	let mut screen_buffer = ScreenBuffer::new();

	rayon::ThreadPoolBuilder::new().num_threads(24).build_global().unwrap();

	loop {
		screen_buffer.check_resize();

		parameters.update();

		let width = screen_buffer.width();
		let height = screen_buffer.height();

		let extents = parameters.extents(width, height);
		screen_buffer
			.buffer
			.par_iter_mut()
			.enumerate()
			.for_each(|(index, pixel)| {
				let x = (index as u32) % width;
				let y = (index as u32) / width;

				let result = mandelbrot::run(
					parameters.iterations,
					x,
					y,
					width,
					height,
					extents,
					parameters.precision,
				);

				pixel.r = result[0];
				pixel.g = result[1];
				pixel.b = result[2];
			});

		screen_buffer.draw();

		let lines = [
			format!("FPS: {}", 1.0 / get_frame_time()),
			format!("X: {}", parameters.center_x),
			format!("Y: {}", parameters.center_y),
			format!("S: {}x", u64::rounding_from(&parameters.scale, RoundingMode::Nearest).0),
			format!("I: {}", parameters.iterations),
		];

		let mut y = 20.0;
		for line in lines {
			draw_text(&line, 5.0, y, 30.0, DARKPURPLE);
			y += 20.0;
		}

		next_frame().await;
	}
}
