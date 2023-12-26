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
	//let max: HashMap<(usize, usize), usize> = HashMap::new();

	let mut paths = vec![Path {
		x: 1,
		y: 1,
		prev: Vec::new(),
		steps: 0,
		prev_dir: Direction::Down,
	}];

	let mut max = 0;

	while paths.len() > 0 {
		let path = paths.pop().unwrap();
		let junctions = walk_to_junction(&map, &path.x, &path.y, &path.prev_dir);
		let mut prev = path.prev.clone();
		let prev_x = match path.prev_dir {
			Direction::Up => path.x,
			Direction::Down => path.x,
			Direction::Left => path.x + 1,
			Direction::Right => path.x - 1,
		};
		let prev_y = match path.prev_dir {
			Direction::Up => path.y + 1,
			Direction::Down => path.y - 1,
			Direction::Left => path.y,
			Direction::Right => path.y,
		};
		prev.push((prev_x, prev_y));
		match junctions {
			None => {
				continue;
			}
			Some((destinations, steps)) => {
				if destinations.len() == 1 {
					if path.steps + steps > max {
						max = path.steps + steps;
					}
					println!(
						"Found path: {}, {}, {:?}, {}",
						path.steps,
						max,
						paths.len(),
						path.prev.len()
					);
				} else {
					for (x, y, prev_dir) in destinations {
						//println!("Adding path: {:?}, {:?}", (x, y), prev_dir);
						let jn_x = match prev_dir {
							Direction::Up => x,
							Direction::Down => x,
							Direction::Left => x + 1,
							Direction::Right => x - 1,
						};
						let jn_y = match prev_dir {
							Direction::Up => y + 1,
							Direction::Down => y - 1,
							Direction::Left => y,
							Direction::Right => y,
						};
						if !prev.contains(&(jn_x, jn_y)) {
							paths.push(Path {
								x,
								y,
								steps: path.steps + steps,
								prev_dir,
								prev: prev.clone(),
							});
						} else {
							continue;
						}
					}
				}
			}
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

fn walk_to_junction(
	map: &Vec<Vec<char>>,
	x: &usize,
	y: &usize,
	prev_dir: &Direction,
) -> Option<(Vec<(usize, usize, Direction)>, usize)> {
	let mut destinations = vec![(x.clone(), y.clone(), prev_dir.clone())];
	let mut steps = 0;
	while destinations.len() == 1 {
		steps += 1;
		let (x, y, prev_dir) = destinations.pop().unwrap();
		let end_x = 139;
		let end_y = 140;
		//let end_x = 21;
		//let end_y = 22;
		if x == end_x && y == end_y {
			return Some((vec![(end_x, end_y, prev_dir)], steps));
		}
		let dirs = match prev_dir {
			Direction::Up => [Direction::Left, Direction::Right, Direction::Up],
			Direction::Down => [Direction::Left, Direction::Right, Direction::Down],
			Direction::Left => [Direction::Up, Direction::Down, Direction::Left],
			Direction::Right => [Direction::Up, Direction::Down, Direction::Right],
		};
		for dir in dirs {
			let c = safe_retrieval(&map, x, y, dir.clone());
			if c.is_none() {
				continue;
			}
			let new_x = match dir {
				Direction::Up => x,
				Direction::Down => x,
				Direction::Left => x - 1,
				Direction::Right => x + 1,
			};
			let new_y = match dir {
				Direction::Up => y - 1,
				Direction::Down => y + 1,
				Direction::Left => y,
				Direction::Right => y,
			};
			destinations.push((new_x, new_y, dir));
		}
	}
	if destinations.len() == 0 {
		//println!("Dead end, {x}, {y}, {prev_dir:?}");
		None
	} else {
		//println!("Junction, {:?}, {}", destinations, steps);
		Some((destinations, steps))
	}
}

fn safe_retrieval(map: &Vec<Vec<char>>, x: usize, y: usize, dir: Direction) -> Option<char> {
	match dir {
		Direction::Up => {
			if y > 0 {
				let c = map[y - 1][x];
				if c != '#' {
					return Some(c);
				}
			}
		}
		Direction::Down => {
			if y < map.len() - 1 {
				let c = map[y + 1][x];
				if c != '#' {
					return Some(c);
				}
			}
		}
		Direction::Left => {
			if x > 0 {
				let c = map[y][x - 1];
				if c != '#' {
					return Some(c);
				}
			}
		}
		Direction::Right => {
			if x < map.len() - 1 {
				let c = map[y][x + 1];
				if c != '#' {
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
	prev: Vec<(usize, usize)>,
	steps: usize,
	prev_dir: Direction,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}
