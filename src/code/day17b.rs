//use std::cmp;
use pheap::PairingHeap;
use std::collections::HashMap;
use std::{fs, vec};

pub fn solution() {
	const END: u8 = 4;
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	//let input = fs::read_to_string("src/inputs/day17.txt").expect("Error reading input");
	let input = parse_input();

	let origin = Node {
		x: 0,
		y: 0,
		dir: Direction::Down,
		straight: 0,
	};

	// new
	let mut distances: HashMap<Node, usize> = HashMap::new();
	distances.insert(origin.clone(), 0);
	let mut heap = PairingHeap::new();
	heap.insert(origin, 0);
	//

	// map out neighbors
	'w: loop {
		// get min node
		let (min_node, min_dist) = heap.delete_min().unwrap();
		for step in vec![Move::N, Move::S, Move::E, Move::W] {
			let neighbor = min_node.go(step, &input);
			if neighbor.is_none() {
				continue;
			}
			let (neighbor, val) = neighbor.unwrap();
			let new_dist = val + min_dist;
			match distances.get(&neighbor) {
				Some(dist) => {
					if new_dist < *dist {
						distances.insert(neighbor.clone(), new_dist);
						heap.insert(neighbor.clone(), new_dist);
					}
				}
				None => {
					distances.insert(neighbor.clone(), new_dist);
					heap.insert(neighbor.clone(), new_dist);
				}
			};
			let x_len = input[0].len();
			let y_len = input.len();
			if neighbor.x == x_len - 1 && neighbor.y == y_len - 1 {
				println!(
					"important node: {:?}, {}. {x_len}, {y_len}",
					neighbor, distances[&neighbor]
				);
				if neighbor.straight >= END {
					break 'w;
				}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
	x: usize,
	y: usize,
	dir: Direction,
	straight: u8,
}

impl Node {
	fn go(&self, step: Move, input: &Vec<Vec<usize>>) -> Option<(Node, usize)> {
		const TURN_MIN: u8 = 4;
		const STRAIGHT_MAX: u8 = 10;
		match step {
			Move::N => {
				if self.dir == Direction::Down {
					return None;
				}
				if self.dir != Direction::Up && self.straight < TURN_MIN {
					return None;
				}
				let val = safe_retrieval(self.x, self.y, 0, -1, input);
				if val.is_none() {
					return None;
				}
				let straight = match self.dir {
					Direction::Up => self.straight + 1,
					_ => 1,
				};
				if straight > STRAIGHT_MAX {
					return None;
				}
				return Some((
					Node {
						x: self.x,
						y: self.y - 1,
						dir: Direction::Up,
						straight,
					},
					val.unwrap(),
				));
			}
			Move::S => {
				if self.dir == Direction::Up {
					return None;
				}
				if self.dir != Direction::Down && self.straight < TURN_MIN {
					return None;
				}
				let val = safe_retrieval(self.x, self.y, 0, 1, input);
				if val.is_none() {
					return None;
				}
				let straight = match self.dir {
					Direction::Down => self.straight + 1,
					_ => 1,
				};
				if straight > STRAIGHT_MAX {
					return None;
				}
				return Some((
					Node {
						x: self.x,
						y: self.y + 1,
						dir: Direction::Down,
						straight,
					},
					val.unwrap(),
				));
			}
			Move::E => {
				if self.dir == Direction::Left {
					return None;
				}
				if self.dir != Direction::Right && self.straight < TURN_MIN {
					return None;
				}
				let val = safe_retrieval(self.x, self.y, 1, 0, input);
				if val.is_none() {
					return None;
				}
				let straight = match self.dir {
					Direction::Right => self.straight + 1,
					_ => 1,
				};
				if straight > STRAIGHT_MAX {
					return None;
				}
				return Some((
					Node {
						x: self.x + 1,
						y: self.y,
						dir: Direction::Right,
						straight,
					},
					val.unwrap(),
				));
			}
			Move::W => {
				if self.dir == Direction::Right {
					return None;
				}
				if self.dir != Direction::Left && self.straight < TURN_MIN {
					return None;
				}
				let val = safe_retrieval(self.x, self.y, -1, 0, input);
				if val.is_none() {
					return None;
				}
				let straight = match self.dir {
					Direction::Left => self.straight + 1,
					_ => 1,
				};
				if straight > STRAIGHT_MAX {
					return None;
				}
				return Some((
					Node {
						x: self.x - 1,
						y: self.y,
						dir: Direction::Left,
						straight,
					},
					val.unwrap(),
				));
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, Clone)]
enum Move {
	N,
	S,
	E,
	W,
}

fn parse_input() -> Vec<Vec<usize>> {
// 	let input = "2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533";
// 	let input = "111111111111
// 999999999991
// 999999999991
// 999999999991
// 999999999991";
	let input = fs::read_to_string("src/inputs/day17.txt").expect("Error reading input");
	let mut map: Vec<Vec<usize>> = Vec::new();
	for line in input.lines() {
		let mut row: Vec<usize> = Vec::new();
		for c in line.chars() {
			row.push(c.to_digit(10).unwrap() as usize);
		}
		map.push(row);
	}
	map
}

fn new_idx(idx: usize, step: isize, max: usize) -> Option<usize> {
	let new_idx = (idx as isize) + step;
	if new_idx < 0 || new_idx >= (max as isize) {
		return None;
	}
	return Some(new_idx as usize);
}

fn safe_retrieval(
	x: usize,
	y: usize,
	x_step: isize,
	y_step: isize,
	input: &Vec<Vec<usize>>,
) -> Option<usize> {
	let y_max = input.len();
	let x_max = input[0].len();
	let new_x = new_idx(x, x_step, x_max);
	let new_y = new_idx(y, y_step, y_max);
	match (new_x, new_y) {
		(Some(new_x), Some(new_y)) => {
			return Some(input[new_y][new_x] as usize);
		}
		_ => {
			return None;
		}
	}
}
