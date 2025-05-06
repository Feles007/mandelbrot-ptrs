use crate::mandelbrot::Precision;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::prelude::get_frame_time;
use malachite::base::num::basic::traits::One;
use malachite::base::num::conversion::traits::RoundingFrom;
use malachite::base::rounding_modes::RoundingMode;
use malachite::Float;

pub struct Parameters {
	pub center_x: Float,
	pub center_y: Float,
	pub scale: Float,
	pub iterations: u32,
	pub precision: Precision,
}
impl Parameters {
	pub fn update(&mut self) {
		let delta = Float::from(get_frame_time());

		let movement = &delta * (Float::ONE / &self.scale);

		if is_key_down(KeyCode::Up) {
			self.center_y -= &movement;
		} else if is_key_down(KeyCode::Down) {
			self.center_y += &movement;
		}

		if is_key_down(KeyCode::Left) {
			self.center_x -= movement;
		} else if is_key_down(KeyCode::Right) {
			self.center_x += movement;
		}

		if is_key_pressed(KeyCode::Minus) {
			self.iterations /= 2;
			if self.iterations == 0 {
				self.iterations = 1;
			}
		} else if is_key_pressed(KeyCode::Equal) {
			self.iterations *= 2;
		}

		let scale_factor = Float::ONE + delta;

		if is_key_down(KeyCode::Z) {
			self.scale *= scale_factor;
		} else if is_key_down(KeyCode::X) {
			self.scale /= scale_factor;
		}

		if is_key_pressed(KeyCode::Key1) {
			self.precision = Precision::F32;
		} else if is_key_pressed(KeyCode::Key2) {
			self.precision = Precision::F64;
		} else if is_key_pressed(KeyCode::Key3) {
			self.precision = Precision::Arbitrary;
		}
	}
	pub fn extents(&self, width: u32, height: u32) -> Extents {
		let scale = f64::rounding_from(&self.scale, RoundingMode::Nearest).0;
		let center_x = f64::rounding_from(&self.center_x, RoundingMode::Nearest).0;
		let center_y = f64::rounding_from(&self.center_y, RoundingMode::Nearest).0;

		let vmin = -scale.recip();
		let vmax = scale.recip();
		let ratio = width as f64 / height as f64;
		let hmin = vmin * ratio + center_x;
		let hmax = vmax * ratio + center_x;

		let offset_vmin = vmin + center_y;
		let offset_vmax = vmax + center_y;

		Extents {
			vmin: offset_vmin,
			vmax: offset_vmax,
			hmin,
			hmax,
		}
	}
}
#[derive(Copy, Clone)]
pub struct Extents {
	pub vmin: f64,
	pub vmax: f64,
	pub hmin: f64,
	pub hmax: f64,
}
