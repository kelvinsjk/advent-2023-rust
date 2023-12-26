//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	// 	let input = "#.#####################
	// #.......#########...###
	// #######.#########.#.###
	// ###.....#.>.>.###.#.###
	// ###v#####.#v#.###.#.###
	// ###.>...#.#.#.....#...#
	// ###v###.#.#.#########.#
	// ###...#.#.#.......#...#
	// #####.#.#.#######.#.###
	// #.....#.#.#.......#...#
	// #.#####.#.#.#########v#
	// #.#...#...#...###...>.#
	// #.#.#v#######v###.###v#
	// #...#.>.#...>.>.#.###.#
	// #####v#.#.###v#.#.###.#
	// #.....#...#...#.#.#...#
	// #.#########.###.#.#.###
	// #...###...#...#...#.###
	// ###.###.#.###v#####v###
	// #...#...#.#.>.>.#.>.###
	// #.###.###.#.###.#.#v###
	// #.....###...###...#...#
	// #####################.#";
	let input: String = fs::read_to_string("src/inputs/day23.txt").expect("Error reading input");
	let map = parse_input(&input);
	let end_x = 139;
	let end_y = 140;
	//let max: HashMap<(usize, usize), usize> = HashMap::new();

	let mut paths = vec![Path {
		x: 1,
		y: 0,
		cell: '.',
		prev: Vec::new(),
		in_progress: true,
	}];

	let mut max = 0;

	'alpha: loop {
		let mut changed = false;
		let mut new_paths: Vec<Path> = Vec::new();
		for path in &mut paths {
			if !path.in_progress {
				continue;
			}
			if path.x == end_x && path.y == end_y {
				println!("Found path: {:?}", path.prev.len());
				path.in_progress = false;
				if path.prev.len() > max {
					max = path.prev.len();
				}
			}
			let mut path_mutated = false;
			let dirs = match path.cell {
				'>' => vec![Direction::Right],
				'v' => vec![Direction::Down],
				'<' => vec![Direction::Left],
				'^' => vec![Direction::Up],
				'.' => vec![
					Direction::Right,
					Direction::Down,
					Direction::Left,
					Direction::Up,
				],
				_ => panic!("Invalid cell"),
			};
			let old_x = path.x.clone();
			let old_y = path.y.clone();
			let prev = path.prev.clone();
			for dir in dirs {
				let c = safe_retrieval(&map, old_x, old_y, dir.clone());
				if c.is_none() {
					continue;
				}
				let new_x = match dir {
					Direction::Up => old_x,
					Direction::Down => old_x,
					Direction::Left => old_x - 1,
					Direction::Right => old_x + 1,
				};
				let new_y = match dir {
					Direction::Up => old_y - 1,
					Direction::Down => old_y + 1,
					Direction::Left => old_y,
					Direction::Right => old_y,
				};
				if path.prev.contains(&(new_x, new_y)) {
					continue;
				}
				let c = c.unwrap();
				changed = true;
				if path_mutated {
					let mut prev = prev.clone();
					prev.push((path.x, path.y));
					new_paths.push(Path {
						x: new_x,
						y: new_y,
						cell: c,
						prev,
						in_progress: true,
					});
				} else {
					path_mutated = true;
					path.push(new_x, new_y, c);
				}
			}
			if !path_mutated {
				path.in_progress = false;
			}
		}
		for new_path in new_paths {
			paths.push(new_path);
		}
		if !changed {
			break 'alpha;
		}
	}

	println!("Final max path: {:#?}", max);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
	let mut map: Vec<Vec<char>> = Vec::new();
	for line in input.lines() {
		let mut chars: Vec<char> = Vec::new();
		for c in line.chars() {
			chars.push(c);
		}
		map.push(chars);
	}
	map
}

fn safe_retrieval(map: &Vec<Vec<char>>, x: usize, y: usize, dir: Direction) -> Option<char> {
	match dir {
		Direction::Up => {
			if y > 0 {
				let c = map[y - 1][x];
				if c != '#' && c != 'v' {
					return Some(c);
				}
			}
		}
		Direction::Down => {
			if y < map.len() - 1 {
				let c = map[y + 1][x];
				if c != '#' && c != '^' {
					return Some(c);
				}
			}
		}
		Direction::Left => {
			if x > 0 {
				let c = map[y][x - 1];
				if c != '#' && c != '>' {
					return Some(c);
				}
			}
		}
		Direction::Right => {
			if x < map.len() - 1 {
				let c = map[y][x + 1];
				if c != '#' && c != '<' {
					return Some(c);
				}
			}
		}
	}
	None
}

#[derive(Debug, Clone)]
struct Path {
	x: usize,
	y: usize,
	cell: char,
	prev: Vec<(usize, usize)>,
	in_progress: bool,
}

impl Path {
	fn push(&mut self, x: usize, y: usize, cell: char) {
		self.prev.push((self.x, self.y));
		self.x = x;
		self.y = y;
		self.cell = cell;
	}
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}
