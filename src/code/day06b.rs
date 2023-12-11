//use std::cmp;
//use std::collections::HashMap;
//use std::fs;

pub fn solution() {
	let t: i64 = 63_78_94_68;
	let d: i64 = 411_1274_2047_1035;
	let a: i64 = 1;
	let b = -1 * t;
	let c = d;
	let discriminant = b * b - 4 * a * c;
	let sqrt_discriminant = (discriminant as f64).sqrt();
	let neg_b = (-1 * b) as f64;
	let den = (2 * a) as f64;
	let t1 = (neg_b - sqrt_discriminant) / (den);
	let t2 = (neg_b + sqrt_discriminant) / (den);
	let ways = t2.floor() - t1.ceil() + 1.;

	// final result
	println!("Product: {}", ways);
}
