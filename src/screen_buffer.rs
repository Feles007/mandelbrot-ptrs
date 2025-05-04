use bytemuck::{cast_slice, Pod, Zeroable};
use macroquad::prelude::*;

pub struct ScreenBuffer {
	pub buffer: Vec<Pixel>,
	width: u32,
	height: u32,
	texture: Texture2D,
}
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Pixel {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}
impl ScreenBuffer {
	pub fn new() -> Self {
		Self::setup()
	}
	fn setup() -> Self {
		let width = screen_width() as u32;
		let height = screen_height() as u32;
		let buffer = vec![
			Pixel {
				r: 0,
				g: 0,
				b: 0,
				a: 255,
			};
			width as usize * height as usize
		];
		let texture = Texture2D::from_rgba8(width as u16, height as u16, cast_slice(&buffer));

		Self {
			buffer,
			width,
			height,
			texture,
		}
	}
	pub fn check_resize(&mut self) {
		if self.width == screen_width() as u32 && self.height == screen_height() as u32 {
			return;
		}
		*self = Self::setup();
	}
	pub fn draw(&self) {
		self.texture
			.update_from_bytes(self.width, self.height, cast_slice(&self.buffer));
		draw_texture(&self.texture, 0.0, 0.0, WHITE);
	}
	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn height(&self) -> u32 {
		self.height
	}
}
