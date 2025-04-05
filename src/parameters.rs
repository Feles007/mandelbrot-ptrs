use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::prelude::get_frame_time;

pub struct Parameters {
	pub center_x: f64,
	pub center_y: f64,
	pub scale: f64,
	pub iterations: u32,
}
impl Parameters {
	pub fn update(&mut self) {
		let delta = get_frame_time() as f64;

		let movement = delta * (1.0 / self.scale);

		if is_key_down(KeyCode::Up) {
			self.center_y -= movement;
		} else if is_key_down(KeyCode::Down) {
			self.center_y += movement;
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

		let scale_factor = 1.0 + delta;

		if is_key_down(KeyCode::Z) {
			self.scale *= scale_factor;
		} else if is_key_down(KeyCode::X) {
			self.scale /= scale_factor;
		}
	}
	pub fn extents(&self, width: u32, height: u32) -> Extents {
		let x_offset = -0.5;

		let vmin = -1.15 * (1.0 / self.scale);
		let vmax = 1.15 * (1.0 / self.scale);
		let ratio = width as f64 / height as f64;
		let hmin = vmin * ratio + x_offset + self.center_x;
		let hmax = vmax * ratio + x_offset + self.center_x;

		let offset_vmin = vmin + self.center_y;
		let offset_vmax = vmax + self.center_y;

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
