use crate::parameters::{Extents, Parameters};
use crate::screen_buffer::ScreenBuffer;
use crate::{get_color, mix};
use rayon::prelude::*;
use std::arch::x86_64::*;
use std::array;

pub fn run(screen_buffer: &mut ScreenBuffer, parameters: &Parameters, extents: &Extents) {
	let width = screen_buffer.width();
	let height = screen_buffer.height();

	screen_buffer
		.buffer
		.par_chunks_exact_mut(4)
		.enumerate()
		.for_each(|(index, pixels)| {
			let sxys: [_; 4] = array::from_fn(|i| {
				let unroll_index = (index * 4 + i) as u32;
				let x_a = ((unroll_index % width) as f64) / (width as f64);
				let y_a = ((unroll_index / width) as f64) / (height as f64);
				(
					mix(extents.hmin, extents.hmax, x_a),
					mix(extents.vmin, extents.vmax, y_a),
				)
			});
			let result = unsafe {
				let cr = _mm256_set_pd(sxys[0].0, sxys[1].0, sxys[2].0, sxys[3].0);
				let ci = _mm256_set_pd(sxys[0].1, sxys[1].1, sxys[2].1, sxys[3].1);
				mandelbrot_f64x4(cr, ci, parameters.iterations)
			};
			let colors: [_; 4] = array::from_fn(|i| get_color(parameters.iterations, result[i]));

			for i in 0..4 {
				pixels[i].r = colors[i][0];
				pixels[i].g = colors[i][1];
				pixels[i].b = colors[i][2];
			}
		});
}
unsafe fn mandelbrot_f64x4(cr: __m256d, ci: __m256d, iterations: u32) -> [u32; 4] {
	let all_fours = _mm256_set1_pd(4.0);

	let mut x = _mm256_setzero_pd();
	let mut y = _mm256_setzero_pd();

	let mut iterations_result = _mm256_set1_epi64x(1);

	let mut add_masked = _mm256_set1_epi64x(1);

	for _ in 0..iterations {
		// Optimization: Break if the mask is empty
		if _mm256_testz_si256(add_masked, add_masked) != 0 {
			break;
		}

		// x^2 and y^2
		let x2 = _mm256_mul_pd(x, x);
		let y2 = _mm256_mul_pd(y, y);

		// x^2 + y^2
		let x2y2 = _mm256_add_pd(x2, y2);

		// If x^2+y^2 > 4, remove the value from the corresponding slot in add_masked
		let mask = _mm256_castpd_si256(_mm256_cmp_pd::<{ _CMP_NGT_UQ }>(x2y2, all_fours));
		add_masked = _mm256_and_si256(mask, add_masked);

		// Only adding the ones that still have a one in add_masked
		iterations_result = _mm256_add_epi64(iterations_result, add_masked);

		let new_x = _mm256_add_pd(_mm256_sub_pd(x2, y2), cr);

		let xy = _mm256_mul_pd(x, y);
		let new_y = _mm256_add_pd(_mm256_add_pd(xy, xy), ci);

		y = new_y;
		x = new_x;
	}

	[
		_mm256_extract_epi64::<3>(iterations_result) as u32,
		_mm256_extract_epi64::<2>(iterations_result) as u32,
		_mm256_extract_epi64::<1>(iterations_result) as u32,
		_mm256_extract_epi64::<0>(iterations_result) as u32,
	]
}
