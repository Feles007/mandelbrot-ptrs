use macroquad::prelude::*;

pub struct ScreenBuffer {
	pub buffer: Vec<u8>,
	width: u32,
	height: u32,
	texture: Texture2D,
}
impl ScreenBuffer {
	pub fn new() -> Self {
		Self::setup()
	}
	fn setup() -> Self {
		let width = screen_width() as u32;
		let height = screen_height() as u32;
		let buffer = vec![0; width as usize * height as usize * 4];
		let texture = Texture2D::from_rgba8(width as u16, height as u16, &buffer);

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
		self.texture.update_from_bytes(self.width, self.height, &self.buffer);
		draw_texture(&self.texture, 0.0, 0.0, WHITE);
	}
	pub fn width(&self) -> u32 {
		self.width
	}
	pub fn height(&self) -> u32 {
		self.height
	}
}
