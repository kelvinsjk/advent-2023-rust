//use std::cmp;
//use std::collections::HashMap;
use std::{fs, vec};

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	//let input = fs::read_to_string("src/inputs/day17.txt").expect("Error reading input");
	let input = parse_input();
	// must be square
	let y_len = input.len();
	let mut max = 0;

	for i in 0..y_len {
		for dir in vec![
			Direction::Up,
			Direction::Down,
			Direction::Left,
			Direction::Right,
		] {
			let energized = get_energized(i, dir, &input);
			if energized > max {
				println!("energized: {}", energized);
				max = energized;
			}
		}
	}
	println!("max: {}", max);
}

fn get_energized(idx: usize, dir: Direction, input: &Vec<Vec<char>>) -> usize {
	let x = 0;
	let y = 0;
	let x_len = input[0].len();
	let y_len = input.len();
	let mut beams: Vec<Node> = Vec::new();

	match dir {
		Direction::Up => {
			let x = idx;
			let val = input[y_len - 1][x];
			let dir = match val {
				'.' => Direction::Up,
				'\\' => Direction::Left,
				'/' => Direction::Right,
				'|' => Direction::Up,
				'-' => {
					beams.push(Node {
						x,
						y,
						dir: Direction::Left,
					});
					Direction::Right
				}
				_ => panic!("Unexpected value: {}", val),
			};
			beams.push(Node { x, y, dir });
		}
		Direction::Down => {
			let x = idx;
			let val = input[0][x];
			let dir = match val {
				'.' => Direction::Down,
				'\\' => Direction::Right,
				'/' => Direction::Left,
				'|' => Direction::Down,
				'-' => {
					beams.push(Node {
						x,
						y,
						dir: Direction::Left,
					});
					Direction::Right
				}
				_ => panic!("Unexpected value: {}", val),
			};
			beams.push(Node { x, y, dir });
		}
		Direction::Left => {
			let y = idx;
			let val = input[y][x_len - 1];
			let dir = match val {
				'.' => Direction::Left,
				'\\' => Direction::Up,
				'/' => Direction::Down,
				'-' => Direction::Left,
				'|' => {
					beams.push(Node {
						x,
						y,
						dir: Direction::Up,
					});
					Direction::Down
				}
				_ => panic!("Unexpected value: {}", val),
			};
			beams.push(Node { x, y, dir });
		}
		Direction::Right => {
			let y = idx;
			let val = input[y][0];
			let dir = match val {
				'.' => Direction::Right,
				'\\' => Direction::Down,
				'/' => Direction::Up,
				'-' => Direction::Right,
				'|' => {
					beams.push(Node {
						x,
						y,
						dir: Direction::Up,
					});
					Direction::Down
				}
				_ => panic!("Unexpected value: {}", val),
			};
			beams.push(Node { x, y, dir });
		}
	}

	let mut visited: Vec<Node> = Vec::new();

	// map out neighbors
	while beams.len() > 0 {
		//println!("beams: {:?}", beams);
		let mut new_beams: Vec<Node> = Vec::new();
		for beam in beams {
			visited.push(beam.clone());
			let result = beam.go(&input);
			match result {
				Some(NodeResult::Node(node)) => {
					if !visited.contains(&node) {
						new_beams.push(node);
					}
				}
				Some(NodeResult::Double((node1, node2))) => {
					if !visited.contains(&node1) {
						new_beams.push(node1);
					}
					if !visited.contains(&node2) {
						new_beams.push(node2);
					}
				}
				None => {}
			}
		}
		beams = new_beams;
	}

	let mut energized: Vec<(usize, usize)> = Vec::new();
	for node in visited {
		let Node { x, y, .. } = node;
		if !energized.contains(&(x, y)) {
			energized.push((x, y));
		}
	}
	//println!("energized: {:?}", energized.len());
	energized.len()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
	x: usize,
	y: usize,
	dir: Direction,
}

impl Node {
	fn go(&self, input: &Vec<Vec<char>>) -> Option<NodeResult> {
		match self.dir {
			Direction::Up => {
				let val = safe_retrieval(self.x, self.y, 0, -1, input);
				if val.is_none() {
					return None;
				}
				let val = val.unwrap();
				let x = self.x;
				let y = self.y - 1;
				if val == '.' || val == '|' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Up,
					}));
				}
				if val == '-' {
					let node1 = Node {
						x,
						y,
						dir: Direction::Left,
					};
					let node2 = Node {
						x,
						y,
						dir: Direction::Right,
					};
					return Some(NodeResult::Double((node1, node2)));
				}
				if val == '/' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Right,
					}));
				}
				if val == '\\' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Left,
					}));
				}
				panic!("Unexpected value: {}", val);
			}
			Direction::Down => {
				let val = safe_retrieval(self.x, self.y, 0, 1, input);
				if val.is_none() {
					return None;
				}
				let val = val.unwrap();
				let x = self.x;
				let y = self.y + 1;
				if val == '.' || val == '|' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Down,
					}));
				}
				if val == '-' {
					let node1 = Node {
						x,
						y,
						dir: Direction::Left,
					};
					let node2 = Node {
						x,
						y,
						dir: Direction::Right,
					};
					return Some(NodeResult::Double((node1, node2)));
				}
				if val == '/' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Left,
					}));
				}
				if val == '\\' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Right,
					}));
				}
				panic!("Unexpected value: {}", val);
			}
			Direction::Left => {
				let val = safe_retrieval(self.x, self.y, -1, 0, input);
				if val.is_none() {
					return None;
				}
				let val = val.unwrap();
				let x = self.x - 1;
				let y = self.y;
				if val == '.' || val == '-' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Left,
					}));
				}
				if val == '|' {
					let node1 = Node {
						x,
						y,
						dir: Direction::Up,
					};
					let node2 = Node {
						x,
						y,
						dir: Direction::Down,
					};
					return Some(NodeResult::Double((node1, node2)));
				}
				if val == '/' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Down,
					}));
				}
				if val == '\\' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Up,
					}));
				}
				panic!("Unexpected value: {}", val);
			}
			Direction::Right => {
				let val = safe_retrieval(self.x, self.y, 1, 0, input);
				if val.is_none() {
					return None;
				}
				let val = val.unwrap();
				let x = self.x + 1;
				let y = self.y;
				if val == '.' || val == '-' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Right,
					}));
				}
				if val == '|' {
					let node1 = Node {
						x,
						y,
						dir: Direction::Up,
					};
					let node2 = Node {
						x,
						y,
						dir: Direction::Down,
					};
					return Some(NodeResult::Double((node1, node2)));
				}
				if val == '/' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Up,
					}));
				}
				if val == '\\' {
					return Some(NodeResult::Node(Node {
						x,
						y,
						dir: Direction::Down,
					}));
				}
				panic!("Unexpected value: {}", val);
			}
		}
	}
}

enum NodeResult {
	Node(Node),
	Double((Node, Node)),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

fn parse_input() -> Vec<Vec<char>> {
	// 	let input = ".|...\\....
	// |.-.\\.....
	// .....|-...
	// ........|.
	// ..........
	// .........\\
	// ..../.\\\\..
	// .-.-/..|..
	// .|....-|.\\
	// ..//.|....";
	let input = fs::read_to_string("src/inputs/day16.txt").expect("Error reading input");
	let mut map: Vec<Vec<char>> = Vec::new();
	for line in input.lines() {
		let mut row: Vec<char> = Vec::new();
		for c in line.chars() {
			row.push(c);
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
	input: &Vec<Vec<char>>,
) -> Option<char> {
	let y_max = input.len();
	let x_max = input[0].len();
	let new_x = new_idx(x, x_step, x_max);
	let new_y = new_idx(y, y_step, y_max);
	match (new_x, new_y) {
		(Some(new_x), Some(new_y)) => {
			return Some(input[new_y][new_x]);
		}
		_ => {
			return None;
		}
	}
}
