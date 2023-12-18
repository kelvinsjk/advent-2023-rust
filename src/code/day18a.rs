//use std::cmp;
//use std::collections::HashMap;
use std::{cmp, fs, vec};

pub fn solution() {
	let input = parse_input();
	const XSTART: u32 = 10;
	const YSTART: u32 = 157;
	let mut x_max = 0 as u32;
	let mut y_max = 0 as u32;
	let mut x_min = u32::MAX;
	let mut y_min = u32::MAX;
	let mut point: (u32, u32) = (XSTART, YSTART);
	for (dir, num) in input.clone() {
		match dir {
			'R' => {
				point.0 += num;
				x_max = cmp::max(x_max, point.0);
			}
			'L' => {
				point.0 -= num;
				x_min = cmp::min(x_min, point.0);
			}
			'U' => {
				point.1 -= num;
				y_min = cmp::min(y_min, point.1);
			}
			'D' => {
				point.1 += num;
				y_max = cmp::max(y_max, point.1);
			}
			_ => {
				panic!("Unexpected direction: {}", dir);
			}
		}
	}
	println!("({}, {}) to ({}, {})", x_min, y_min, x_max, y_max);
	let mut map: Vec<Vec<char>> = vec![vec!['.'; (x_max + 1) as usize]; (y_max + 1) as usize];
	point = (XSTART, YSTART);
	let mut prev = 'U';
	for (dir, num) in input {
		let (x, y) = point.clone();
		match (prev, dir) {
			('U', 'R') => {
				map[y as usize][x as usize] = 'F';
			}
			('U', 'L') => {
				map[y as usize][x as usize] = '7';
			}
			('D', 'R') => {
				map[y as usize][x as usize] = 'L';
			}
			('D', 'L') => {
				map[y as usize][x as usize] = 'J';
			}
			('R', 'U') => {
				map[y as usize][x as usize] = 'J';
			}
			('R', 'D') => {
				map[y as usize][x as usize] = '7';
			}
			('L', 'U') => {
				map[y as usize][x as usize] = 'L';
			}
			('L', 'D') => {
				map[y as usize][x as usize] = 'F';
			}
			_ => {
				panic!("Unexpected direction: ({}, {})", prev, dir);
			}
		}
		prev = dir;
		match dir {
			'R' => {
				point.0 += num;
			}
			'L' => {
				point.0 -= num;
			}
			'U' => {
				point.1 -= num;
			}
			'D' => {
				point.1 += num;
			}
			_ => {
				panic!("Unexpected direction: {}", dir);
			}
		}
		for i in 1..num {
			match dir {
				'R' => {
					map[y as usize][(x + i) as usize] = '-';
				}
				'L' => {
					map[y as usize][(x - i) as usize] = '-';
				}
				'U' => {
					map[(y - i) as usize][x as usize] = '|';
				}
				'D' => {
					map[(y + i) as usize][x as usize] = '|';
				}
				_ => {
					panic!("Unexpected direction: {}", dir);
				}
			}
		}
	}
	println!("{:?}", point);
	let mut str = String::new();

	for line in &map {
		//println!("{:?}", line);
		str += &line.iter().collect::<String>();
		str += "\n";
	}

	//fs::write("src/outputs/day18a.txt", str).expect("Unable to write file");

	let x_len = (x_max + 1) as usize;
	let y_len = (y_max + 1) as usize;
	let mut inside = 0;
	let mut pipes = 0;
	for i in 0..x_len {
		for j in 0..y_len {
			let char = map[j][i];
			if char != '.' {
				pipes += 1;
				continue;
			}
			let mut count = 0;
			for k in i..(x_len) {
				let char = map[j][k];
				if char == '|' || char == 'J' || char == 'L' {
					count += 1;
				}
			}
			if (count % 2) == 1 {
				//println!("({}, {}), crossed {} times", i, j, count);
				inside += 1;
			}
		}
	}
	println!(
		"{} by {}, pipe_len: {}, inside: {}. area: {}",
		x_len,
		y_len,
		pipes,
		inside,
		pipes + inside
	);
}

fn parse_input() -> Vec<(char, u32)> {
	// 	let input = "R 6 (#70c710)
	// D 5 (#0dc571)
	// L 2 (#5713f0)
	// D 2 (#d2c081)
	// R 2 (#59c680)
	// D 2 (#411b91)
	// L 5 (#8ceee2)
	// U 2 (#caa173)
	// L 1 (#1b58a2)
	// U 2 (#caa171)
	// R 2 (#7807d2)
	// U 3 (#a77fa3)
	// L 2 (#015232)
	// U 2 (#7a21e3)";
	let input = fs::read_to_string("src/inputs/day18.txt").expect("Error reading input");
	let mut vec: Vec<(char, u32)> = Vec::new();
	for line in input.lines() {
		let (left, _) = line.split_once(" (").unwrap();
		let chars = left.chars();
		let dir = chars.clone().nth(0).unwrap();
		let num = left[2..].parse::<u32>().unwrap();
		vec.push((dir, num));
	}
	vec
}
