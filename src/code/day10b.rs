//use std::cmp;
//use std::collections::HashMap;
use std::fs;
use std::ops::Index;

// slow solution: consider improving algorithm

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day10.txt").expect("Error reading input");
	// 	let input = "FF7FSF7F7F7F7F7F---7
	// L|LJ||||||||||||F--J
	// FL-7LJLJ||||||LJL-77
	// F--JF--7||LJLJIF7FJ-
	// L---JF-JLJIIIIFJLJJ7
	// |F|F-JF---7IIIL7L|7|
	// |FFJF7L7F-JF7IIL---7
	// 7-L-JL7||F7|L7F-7F7|
	// L.L7LFJ|||||FJL7||LJ
	// L7JLJL-JLJLJL--JLJ.L";
	let mut dir = Direction::E;
	// dir manually found: consider running code to determine the two possible starting directions

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

	let mut map: Vec<Vec<char>> = Vec::new();
	let mut s_pos = (0, 0);
	let row_len = input.lines().collect::<Vec<&str>>().len();
	let mut col_len = 0;

	// set up map, get starting position
	for (i, line) in input.lines().enumerate() {
		if i == 0 {
			col_len = line.len();
		}

		let line_chars = line.chars().collect::<Vec<char>>();
		if line_chars.contains(&'S') {
			let pos = line_chars.iter().position(|&x| x == 'S').unwrap();
			s_pos = (i, pos);
		}
		map.push(line.chars().collect());
	}

	let mut pipes: Vec<(usize, usize)> = Vec::new();
	let mut pos = s_pos.clone();
	// follow pipe
	loop {
		pipes.push(pos.clone());
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

	// collect outer-tiles (not enclosed by pipes) by spreading
	let mut inside = 0;
	for i in 0..row_len {
		for j in 0..col_len {
			if pipes.contains(&(i, j)) {
				continue;
			}
			let mut count = 0;
			for k in j..col_len {
				if pipes.contains(&(i, k)) {
					let char = map[i][k];
					if char == '|' || char == 'J' || char == 'L' {
						count += 1;
					}
				}
			}
			if (count % 2) == 1 {
				println!("({}, {}), crossed {} times", i, j, count);
				inside += 1;
			}
		}
	}

	// final result
	println!(
		"{} by {}, pipe_len: {}, inside: {}",
		row_len,
		col_len,
		pipes.len(),
		inside
	);
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
