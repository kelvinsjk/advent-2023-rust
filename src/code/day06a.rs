//use std::cmp;
//use std::collections::HashMap;
//use std::fs;

pub fn solution() {
	let inputs = [(63, 411), (78, 1274), (94, 2047), (68, 1035)];
	// (t,d)

	// x * (t-x) \geq d
	// x^2 - tx + d \leq 0
	// ceil(t1) \leq x \leq floor(t2)
	// ways = floor - ceil + 1

	let mut prod = 1.;
	for (t, d) in inputs.iter() {
		let a: f64 = 1.0;
		let b = -t as f64;
		let c = *d as f64;
		let discriminant = b * b - 4. * a * c;
		let sqrt_discriminant = (discriminant as f64).sqrt();
		let t1 = (-b + sqrt_discriminant) / (2. * a);
		let t2 = (-b - sqrt_discriminant) / (2. * a);
		let ways = t2.floor() - t1.ceil() + 1.;
		prod *= ways;
	}

	// final result
	println!("Product: {}", prod);
}
