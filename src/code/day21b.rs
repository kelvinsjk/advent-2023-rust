//use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
	let input: String = fs::read_to_string("src/inputs/day21.txt").expect("Error reading input");
	let (map, (x, y), _) = parse_input(&input);
	let mut coords: Vec<(usize, usize, isize, isize)> = vec![(x, y, 0, 0)];
	let mut odds: Vec<(usize, usize, isize, isize)> = Vec::new();
	let mut evens: Vec<(usize, usize, isize, isize)> = Vec::new();
	let mut prev: Vec<(usize, usize, isize, isize)> = Vec::new();
	println!("Starting at {:?}, {}, {}", x, y, map.len());
	for i in 0..(65 + 0 * 131) {
		let mut new_coords: Vec<(usize, usize, isize, isize)> = Vec::new();
		println!("Coords: {:?} at step {i}", coords.len());
		while coords.len() > 0 {
			let (x, y, zx, zy) = coords.pop().unwrap();
			for dir in [
				Direction::Up,
				Direction::Down,
				Direction::Left,
				Direction::Right,
			]
			.into_iter()
			{
				if let Some((new_x, new_y, c, z_x, z_y)) = safe_retrieval((x, y), (zx, zy), dir, &map) {
					// if c == '.' || c == 'S' {
					// 	if i % 2 == 0 {
					// 		if !evens.contains(&(new_x, new_y, z_x, z_y)) {
					// 			evens.push((new_x, new_y, z_x, z_y));
					// 			new_coords.push((new_x, new_y, z_x, z_y));
					// 		}
					// 	} else {
					// 		if !odds.contains(&(new_x, new_y, z_x, z_y)) {
					// 			odds.push((new_x, new_y, z_x, z_y));
					// 			new_coords.push((new_x, new_y, z_x, z_y));
					// 		}
					// 	}
					// }
					if c == '.' || c == 'S' {
						if !prev.contains(&(new_x, new_y, z_x, z_y)) {
							prev.push((new_x, new_y, z_x, z_y));
							new_coords.push((new_x, new_y, z_x, z_y));
						}
					}
				}
			}
		}
		coords = new_coords.clone();
	}
	//println!("{:?}", coords);
	println!(
		"{},{}",
		coords.len() + odds.len(),
		coords.len() + evens.len()
	);
}

fn parse_input(
	input: &str,
) -> (
	Vec<Vec<char>>,
	(usize, usize),
	HashMap<(usize, usize), bool>,
) {
	let mut x = 0;
	let mut y = 0;
	let mut map: Vec<Vec<char>> = Vec::new();
	let mut corners: HashMap<(usize, usize), bool> = HashMap::new();
	for (i, line) in input.lines().enumerate() {
		let mut row: Vec<char> = Vec::new();
		let len = line.len();
		for (j, char) in line.chars().enumerate() {
			row.push(char);
			if char == 'S' {
				x = j;
				y = i;
			}
			if (i == 0 || i == len - 1) || (j == 0 || j == len - 1) {
				if char == '.' {
					corners.insert((i, j), false);
				}
			}
		}
		map.push(row);
	}
	(map, (x, y), corners)
}

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

fn safe_retrieval(
	coordinates: (usize, usize),
	z_coordinates: (isize, isize),
	dir: Direction,
	input: &Vec<Vec<char>>,
) -> Option<(usize, usize, char, isize, isize)> {
	let x_len = input[0].len();
	let y_len = input.len();
	let (x, y) = coordinates;
	let (zx, zy) = z_coordinates;
	match dir {
		Direction::Up => {
			if y == 0 {
				Some((x, y_len - 1, input[y_len - 1][x], zx, zy - 1))
			} else {
				Some((x, y - 1, input[y - 1][x], zx, zy))
			}
		}
		Direction::Down => {
			if y == y_len - 1 {
				Some((x, 0, input[0][x], zx, zy + 1))
			} else {
				Some((x, y + 1, input[y + 1][x], zx, zy))
			}
		}
		Direction::Left => {
			if x == 0 {
				Some((x_len - 1, y, input[y][x_len - 1], zx - 1, zy))
			} else {
				Some((x - 1, y, input[y][x - 1], zx, zy))
			}
		}
		Direction::Right => {
			if x == x_len - 1 {
				Some((0, y, input[y][0], zx + 1, zy))
			} else {
				Some((x + 1, y, input[y][x + 1], zx, zy))
			}
		}
	}
}
