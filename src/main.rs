mod methods;
mod parameters;
mod screen_buffer;

use crate::parameters::{Method, Parameters};
use crate::screen_buffer::ScreenBuffer;
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;
use malachite::base::num::conversion::traits::RoundingFrom;
use malachite::base::rounding_modes::RoundingMode;
use malachite::Float;

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
		// F64 max precision
		// center_x: Float::from(-1.5),
		// center_y: Float::from(0.0),
		// scale: Float::from(100000000000000.0),
		center_x: Float::from(-0.5),
		center_y: Float::from(0.0),
		scale: Float::from(1.0),
		iterations: 1024,
		method: Method::Testing,
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

		match parameters.method {
			Method::F32 => methods::f32::run(&mut screen_buffer, &parameters, &extents),
			Method::F64 => methods::f64::run(&mut screen_buffer, &parameters, &extents),
			Method::Arbitrary => methods::arbitrary::run(&mut screen_buffer, &parameters, &extents),
			Method::Testing => methods::f64x4::run(&mut screen_buffer, &parameters, &extents),
		}

		screen_buffer.draw();

		let lines = [
			format!("FPS: {}", 1.0 / get_frame_time()),
			format!(
				"X: {}",
				f64::rounding_from(&parameters.center_x, RoundingMode::Nearest).0
			),
			format!(
				"Y: {}",
				f64::rounding_from(&parameters.center_y, RoundingMode::Nearest).0
			),
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
