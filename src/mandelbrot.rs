use crate::parameters::Extents;

fn mandelbrot(cr: f32, ci: f32, iterations: u32) -> u32 {
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
fn mix(x: f32, y: f32, a: f32) -> f32 {
	x * (1.0 - a) + y * a
}
pub fn run(iterations: u32, x: u32, y: u32, width: u32, height: u32, extents: Extents) -> [u8; 3] {
	let scaled_x = mix(extents.hmin as f32, extents.hmax as f32, (x as f32) / (width as f32));
	let scaled_y = mix(extents.vmin as f32, extents.vmax as f32, (y as f32) / (height as f32));
	let i = mandelbrot(scaled_x, scaled_y, iterations);

	let h = mix(0.0, 359.0, (i as f32) / (iterations as f32));

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
