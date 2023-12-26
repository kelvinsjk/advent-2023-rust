//use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	// 	let input = "1,0,1~1,2,1
	// 0,0,2~2,0,2
	// 0,2,3~2,2,3
	// 0,0,4~0,2,4
	// 2,0,5~2,2,5
	// 0,1,6~2,1,6
	// 1,1,8~1,1,9";
	let input: String = fs::read_to_string("src/inputs/day22.txt").expect("Error reading input");
	let (map, max, size) = parse_input(&input);
	// identify each block by 100*z + 10*y + x; (assumption: single digit x and y)
	let mut filled: HashMap<(usize, usize, usize), usize> = HashMap::new();
	let mut critical: Vec<usize> = Vec::new();
	let mut supporting_map: HashMap<usize, Vec<usize>> = HashMap::new();
	let mut supporter_map: HashMap<usize, Vec<usize>> = HashMap::new();
	for z in 1..max + 1 {
		for (x, y, dir, len) in map.get(&z).unwrap_or(&Vec::new()) {
			// create vector of coordinates
			let mut coords = vec![(*x, *y)];
			let identifier = 100 * z + 10 * y + x;
			match dir {
				Direction::X => {
					for i in 1..*len + 1 {
						coords.push((x + i, *y));
					}
				}
				Direction::Y => {
					for i in 1..*len + 1 {
						coords.push((*x, y + i));
					}
				}
				Direction::Z => {}
			}
			let mut supports: Vec<usize> = Vec::new();
			let mut rest_z = 0;
			for dz in 1..z {
				for (x1, y1) in coords.clone() {
					let space = filled.get(&(x1, y1, z - dz));
					if space.is_some() {
						let support = space.unwrap();
						if !supports.contains(support) {
							supports.push(*support);
						}
					}
				}
				if supports.len() > 0 {
					rest_z = z - dz;
					break;
				}
			}

			if supports.len() == 1 {
				let support = supports[0];
				if !critical.contains(&support) {
					critical.push(support);
				}
			}
			// update supporter/supporting map
			supporter_map.insert(identifier, supports.clone());
			for support in supports {
				let vec = supporting_map.entry(support).or_insert(Vec::new());
				vec.push(identifier);
			}
			// update filled map
			for (x1, y1) in coords {
				let leftover = filled.insert((x1, y1, rest_z + 1), identifier);
				if leftover.is_some() {
					println!("Overwriting {:?} with {}.", leftover.unwrap(), identifier);
				}
			}
			if dir == &Direction::Z {
				for dz in 1..*len + 1 {
					let leftover = filled.insert((*x, *y, rest_z + 1 + dz), identifier);
					if leftover.is_some() {
						println!("Overwriting {:?} with {}.", leftover.unwrap(), identifier);
					}
				}
			}
			//println!("Filled: {:#?}.", filled);
		}
	}
	println!("Answer: {}.", size - critical.len());
	let mut sum = 0;
	for critical in critical {
		let mut disintegrated = vec![critical];
		'alpha: loop {
			let prev_size = disintegrated.len();
			for block in supporter_map.keys() {
				let supporters = supporter_map.get(block).unwrap();
				if supporters.len() > 0
					&& supporters
						.iter()
						.all(|supporter| disintegrated.contains(supporter))
				{
					if !disintegrated.contains(block) {
						disintegrated.push(*block);
					}
				}
			}
			if disintegrated.len() == prev_size {
				break 'alpha;
			}
		}
		sum += disintegrated.len() - 1;
		println!("Sum: {:#?}.", sum);
	}
}

fn parse_input(
	input: &str,
) -> (
	HashMap<usize, Vec<(usize, usize, Direction, usize)>>,
	usize,
	usize,
) {
	let mut map = HashMap::new();
	let mut max = 0;
	for line in input.lines() {
		let (left, right) = line.split_once('~').unwrap();
		let (x, yz) = left.split_once(',').unwrap();
		let (y, z) = yz.split_once(',').unwrap();
		let (x, y, z) = (
			x.parse::<usize>().unwrap(),
			y.parse::<usize>().unwrap(),
			z.parse::<usize>().unwrap(),
		);
		if z > max {
			max = z;
		}
		let vec = map.entry(z).or_insert(Vec::new());
		let (x1, yz1) = right.split_once(',').unwrap();
		let (y1, z1) = yz1.split_once(',').unwrap();
		let (x1, y1, z1) = (
			x1.parse::<usize>().unwrap(),
			y1.parse::<usize>().unwrap(),
			z1.parse::<usize>().unwrap(),
		);
		if x1 != x {
			vec.push((x, y, Direction::X, x1 - x));
		} else if y1 != y {
			vec.push((x, y, Direction::Y, y1 - y));
		} else if z1 != z {
			vec.push((x, y, Direction::Z, z1 - z));
		} else {
			// singleton block
			vec.push((x, y, Direction::Z, 0));
		}
	}
	(map, max, input.lines().count())
}

#[derive(Debug, PartialEq)]
enum Direction {
	X,
	Y,
	Z,
}
