//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	let input = parse_input();
	let mut point: (i64, i64) = (0, 0);
	let mut prev: (i64, i64);
	let mut sum: i64 = 0;
	let mut pipe_area: i64 = 0;
	let mut pipes: i64 = 0;
	let mut prev_dir = 'U';

	for (dir, num) in input.clone() {
		prev = point.clone();
		pipe_area += (num - 1) * 2;
		match (prev_dir, dir) {
			('U', 'R') | ('R', 'D') | ('D', 'L') | ('L', 'U') => {
				pipe_area += 1;
			}
			('U', 'L') | ('L', 'D') | ('D', 'R') | ('R', 'U') => {
				pipe_area += 3;
			}
			_ => panic!("Unexpected direction: {} {}", prev_dir, dir),
		}
		prev_dir = dir;
		pipes += num;
		match dir {
			'R' => {
				point.0 += num * 2;
			}
			'L' => {
				point.0 -= num * 2;
			}
			'U' => {
				point.1 -= num * 2;
			}
			'D' => {
				point.1 += num * 2;
			}
			_ => {
				panic!("Unexpected direction: {}", dir);
			}
		}
		sum += prev.0 * point.1 - prev.1 * point.0;
	}
	println!(
		"{sum}/2 = {}, {}, {:?}, {}",
		sum / 2,
		pipe_area,
		point,
		((sum / 2) - pipe_area) / 4 + pipes
	);
}

fn parse_input() -> Vec<(char, i64)> {
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
	let mut vec: Vec<(char, i64)> = Vec::new();
	for line in input.lines() {
		let (_, right) = line.split_once(" (#").unwrap();
		let hexa = &right[..5];
		let dir = right[5..6].chars().nth(0).unwrap();
		let dir = match dir {
			'0' => 'R',
			'1' => 'D',
			'2' => 'L',
			'3' => 'U',
			_ => panic!("Unexpected direction: {}", dir),
		};
		let num = i64::from_str_radix(&hexa.to_string(), 16).unwrap();
		//println!("{} {} {}", hexa, dir, num);
		vec.push((dir, num));
	}
	vec
}
