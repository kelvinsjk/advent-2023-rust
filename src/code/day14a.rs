//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day14.txt").expect("Error reading input");
	// 	let input = "OOOO.#.O..
	// OO..#....#
	// OO..O##..O
	// O..#.OO...
	// ........#.
	// ..#....#.#
	// ..O..#.O.O
	// ..O.......
	// #....###..
	// #....#....";

	let mut sum = 0;
	let row_len = input.lines().count() as i32;
	let col_len = input.lines().next().unwrap().len();
	let mut last_cube = vec![-1; col_len];
	let mut rock_count = vec![0; col_len];

	let k = 2;

	let mut row_idx: i32 = 0;
	for row in input.lines() {
		for (i, char) in row.chars().enumerate() {
			match char {
				'.' => (),
				'O' => rock_count[i] += 1,
				'#' => {
					let a = row_len - last_cube[i].clone() - 1;
					let n = rock_count[i].clone();
					let s_n = n * (2 * a + (n - 1) * -1) / 2;
					sum += s_n;
					last_cube[i] = row_idx;
					rock_count[i] = 0;
					if i == k {
						println!("s_n: {}", s_n)
					}
				}
				_ => panic!("Invalid char"),
			}
		}
		row_idx += 1;
	}
	for i in 0..col_len {
		let a = row_len - last_cube[i].clone() - 1;
		let n = rock_count[i].clone();
		let s_n = n * (2 * a + (n - 1) * -1) / 2;
		sum += s_n;
		if i == k {
			println!("s_n: {}, {a}, {n}, {}", s_n, last_cube[i])
		}
	}

	// final result
	println!("Sum: {}", sum);
}
