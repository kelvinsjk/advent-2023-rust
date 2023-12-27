//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	// 	let input = "19, 13, 30 @ -2,  1, -2
	// 18, 19, 22 @ -1, -1, -2
	// 20, 25, 34 @ -2, -2, -4
	// 12, 31, 28 @ -1, -2, -1
	// 20, 19, 15 @  1, -5, -3";
	let input: String = fs::read_to_string("src/inputs/day24.txt").expect("Error reading input");
	let hailstones = parse_input(&input);
	let mut solns = 0;
	let len = hailstones.len();
	for i in 0..len {
		for j in i + 1..len {
			if let Some(soln) = solve(&hailstones[i], &hailstones[j]) {
				println!("{:?}", soln);
				solns += 1;
			}
		}
	}
	println!("Final solutions: {}", solns);
}

fn solve(one: &[[isize; 3]; 2], two: &[[isize; 3]; 2]) -> Option<[f64; 4]> {
	// assume no infinite intersection?
	// assume no zero gradient
	let x0_1 = one[0][0];
	let y0_1 = one[0][1];
	//let z0_1 = one[0][2];
	let mx_1 = one[1][0];
	let my_1 = one[1][1];
	//let mz_1 = one[1][2];
	let x0_2 = two[0][0];
	let y0_2 = two[0][1];
	//let z0_2 = two[0][2];
	let mx_2 = two[1][0];
	let my_2 = two[1][1];
	//let mz_2 = two[1][2];
	let m1 = my_1 as f64 / mx_1 as f64;
	let m2 = my_2 as f64 / mx_2 as f64;
	let diff = m1 - m2;
	if diff.abs() < f64::EPSILON {
		return None;
	}
	let x = (y0_2 as f64 - y0_1 as f64 + m1 * x0_1 as f64 - m2 * x0_2 as f64) / diff;
	let y = m1 * (x - x0_1 as f64) + y0_1 as f64;
	let t1 = (x - x0_1 as f64) / mx_1 as f64;
	let t2 = (x - x0_2 as f64) / mx_2 as f64;
	let min = 200000000000000.0; //7.0;
	let max = 400000000000000.0; //27.0;
	if t1 < 0.0 || t2 < 0.0 || x < min || x > max || y < min || y > max {
		return None;
	}
	Some([x, y, t1, t2])
}

fn parse_input(input: &str) -> Vec<[[isize; 3]; 2]> {
	let mut hailstones: Vec<[[isize; 3]; 2]> = Vec::new();
	for line in input.lines() {
		let (left, right) = line.split_once(" @ ").unwrap();
		let (x, yz) = left.split_once(", ").unwrap();
		let (y, z) = yz.split_once(", ").unwrap();
		let x = x.trim().parse::<isize>().unwrap();
		let y = y.trim().parse::<isize>().unwrap();
		let z = z.trim().parse::<isize>().unwrap();
		let start = [x, y, z];
		let (x, yz) = right.split_once(", ").unwrap();
		let (y, z) = yz.split_once(", ").unwrap();
		let x = x.trim().parse::<isize>().unwrap();
		let y = y.trim().parse::<isize>().unwrap();
		let z = z.trim().parse::<isize>().unwrap();
		hailstones.push([start, [x, y, z]]);
	}
	hailstones
}
