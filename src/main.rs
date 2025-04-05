mod mandelbrot;
mod parameters;
mod screen_buffer;

use crate::parameters::Parameters;
use crate::screen_buffer::ScreenBuffer;
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;
use rayon::prelude::*;

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
		center_x: 0.0,
		center_y: 0.0,
		scale: 1.0,
		iterations: 1024,
		precise: false,
	};

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
			.par_chunks_exact_mut(4)
			.enumerate()
			.for_each(|(index, slice)| {
				let x = (index as u32) % width;
				let y = (index as u32) / width;

				let result = mandelbrot::run(parameters.iterations, x, y, width, height, extents, parameters.precise);

				slice[0] = result[0];
				slice[1] = result[1];
				slice[2] = result[2];
				slice[3] = 255;
			});

		screen_buffer.draw();

		let lines = [
			format!("FPS: {}", 1.0 / get_frame_time()),
			format!("X: {}", parameters.center_x),
			format!("Y: {}", parameters.center_y),
			format!("S: {}x", parameters.scale as u64),
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
