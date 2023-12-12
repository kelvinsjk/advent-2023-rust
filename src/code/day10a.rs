//use std::cmp;
//use std::collections::HashMap;
use std::fs;
use std::ops::Index;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day10.txt").expect("Error reading input");
	// 	let input = "7-F7-
	// .FJ|7
	// SJLL7
	// |F--J
	// LJ.LJ";

	// set up
	let pipe = Map {
		n: Direction::N,
		e: Direction::Invalid,
		s: Direction::S,
		w: Direction::Invalid,
	};
	let dash = Map {
		n: Direction::Invalid,
		e: Direction::E,
		s: Direction::Invalid,
		w: Direction::W,
	};
	let l = Map {
		n: Direction::Invalid,
		e: Direction::Invalid,
		s: Direction::E,
		w: Direction::N,
	};
	let j = Map {
		n: Direction::Invalid,
		e: Direction::N,
		s: Direction::W,
		w: Direction::Invalid,
	};
	let seven = Map {
		n: Direction::W,
		e: Direction::S,
		s: Direction::Invalid,
		w: Direction::Invalid,
	};
	let f = Map {
		n: Direction::E,
		e: Direction::Invalid,
		s: Direction::Invalid,
		w: Direction::S,
	};
	let icon_map = IconMap {
		pipe,
		dash,
		l,
		j,
		seven,
		f,
	};

	let mut moves: u32 = 0;
	let mut map: Vec<Vec<char>> = Vec::new();
	let mut s_pos = (0, 0);

	// set up map, get starting position
	for (i, line) in input.lines().enumerate() {
		let line_chars = line.chars().collect::<Vec<char>>();
		if line_chars.contains(&'S') {
			let pos = line_chars.iter().position(|&x| x == 'S').unwrap();
			s_pos = (i, pos);
		}
		map.push(line.chars().collect());
	}

	// dir manually found: consider running code to determine the two possible starting directions
	let mut dir = Direction::E;
	let mut pos = s_pos.clone();
	loop {
		moves += 1;
		// move
		match dir {
			Direction::N => {
				pos.0 -= 1;
			}
			Direction::E => {
				pos.1 += 1;
			}
			Direction::S => {
				pos.0 += 1;
			}
			Direction::W => {
				pos.1 -= 1;
			}
			Direction::Invalid => {
				panic!("Invalid direction");
			}
		};
		// get new direction
		let char = map[pos.0][pos.1];
		if char == 'S' {
			break;
		}
		let new_dir = match char {
			'|' => &icon_map.pipe[&dir],
			'-' => &icon_map.dash[&dir],
			'L' => &icon_map.l[&dir],
			'J' => &icon_map.j[&dir],
			'7' => &icon_map.seven[&dir],
			'F' => &icon_map.f[&dir],
			_ => panic!("Invalid move"),
		};
		dir = new_dir.clone();
	}

	// final result
	println!("Moves: {}, Furthest: {}", moves, moves / 2);
}

#[derive(Clone)]
enum Direction {
	N,
	E,
	S,
	W,
	Invalid,
}

struct IconMap {
	pipe: Map,
	dash: Map,
	l: Map,
	j: Map,
	seven: Map,
	f: Map,
}

struct Map {
	n: Direction,
	e: Direction,
	s: Direction,
	w: Direction,
}

impl Index<&Direction> for Map {
	type Output = Direction;

	fn index(&self, dir: &Direction) -> &Direction {
		match dir {
			Direction::N => &self.n,
			Direction::E => &self.e,
			Direction::S => &self.s,
			Direction::W => &self.w,
			Direction::Invalid => panic!("Invalid direction"),
		}
	}
}
