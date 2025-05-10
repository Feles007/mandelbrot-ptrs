use malachite::base::num::basic::traits::{Two, Zero};
use malachite::Float;
use std::arch::x86_64::{
	_mm256_add_epi64, _mm256_add_pd, _mm256_and_si256, _mm256_castpd_si256, _mm256_cmp_pd, _mm256_extract_epi64,
	_mm256_mul_pd, _mm256_set1_epi64x, _mm256_set1_pd, _mm256_set_pd, _mm256_setzero_pd, _mm256_sub_pd,
	_mm256_testz_si256, _CMP_NGT_UQ,
};

pub fn mandelbrot_f32(cr: f32, ci: f32, iterations: u32) -> u32 {
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
pub fn mandelbrot_f64(cr: f64, ci: f64, iterations: u32) -> u32 {
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
pub fn mandelbrot_precise(cr: Float, ci: Float, iterations: u32) -> u32 {
	let mut x = Float::ZERO;
	let mut y = Float::ZERO;

	for i in 0..iterations {
		let x2 = &x * &x;
		let y2 = &y * &y;
		if &x2 + &y2 > 4 {
			return i;
		}
		let new_x = x2 - y2 + &cr;
		y = Float::TWO * x * y + &ci;
		x = new_x;
	}
	iterations
}
pub unsafe fn mandelbrot_f64x4(cr: [f64; 4], ci: [f64; 4], iterations: u32) -> [u32; 4] {
	let all_fours = _mm256_set1_pd(4.0);

	let cr = _mm256_set_pd(cr[0], cr[1], cr[2], cr[3]);
	let ci = _mm256_set_pd(ci[0], ci[1], ci[2], ci[3]);

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
