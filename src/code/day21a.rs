//use std::cmp;
use std::fs;

pub fn solution() {
	// 	let input = "...........
	// .....###.#.
	// .###.##..#.
	// ..#.#...#..
	// ....#.#....
	// .##..S####.
	// .##..#...#.
	// .......##..
	// .##.#.####.
	// .##..##.##.
	// ...........";
	let input: String = fs::read_to_string("src/inputs/day21.txt").expect("Error reading input");
	let (map, (x, y)) = parse_input(&input);
	let mut coords: Vec<(usize, usize)> = vec![(x, y)];
	println!("Starting at {:?}, {}", x, y);

	for _ in 0..6 {
		let mut new_coords: Vec<(usize, usize)> = Vec::new();
		while coords.len() > 0 {
			let (x, y) = coords.pop().unwrap();
			for dir in [
				Direction::Up,
				Direction::Down,
				Direction::Left,
				Direction::Right,
			]
			.into_iter()
			{
				if let Some((new_x, new_y, c)) = safe_retrieval((x, y), dir, &map) {
					if c == '.' || c == 'S' {
						if !new_coords.contains(&(new_x, new_y)) {
							new_coords.push((new_x, new_y));
						}
					}
				}
			}
		}
		coords = new_coords.clone();
	}
	println!("{:?}", coords);
	println!("{}", coords.len());
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
	let mut x = 0;
	let mut y = 0;
	let mut map: Vec<Vec<char>> = Vec::new();
	for (i, line) in input.lines().enumerate() {
		let mut row: Vec<char> = Vec::new();
		for (j, char) in line.chars().enumerate() {
			row.push(char);
			if char == 'S' {
				x = j;
				y = i;
			}
		}
		map.push(row);
	}
	(map, (x, y))
}

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

fn safe_retrieval(
	coordinates: (usize, usize),
	dir: Direction,
	input: &Vec<Vec<char>>,
) -> Option<(usize, usize, char)> {
	let x_len = input[0].len();
	let y_len = input.len();
	let (x, y) = coordinates;
	match dir {
		Direction::Up => {
			if y == 0 {
				None
			} else {
				Some((x, y - 1, input[y - 1][x]))
			}
		}
		Direction::Down => {
			if y == y_len - 1 {
				None
			} else {
				Some((x, y + 1, input[y + 1][x]))
			}
		}
		Direction::Left => {
			if x == 0 {
				None
			} else {
				Some((x - 1, y, input[y][x - 1]))
			}
		}
		Direction::Right => {
			if x == x_len - 1 {
				None
			} else {
				Some((x + 1, y, input[y][x + 1]))
			}
		}
	}
}
