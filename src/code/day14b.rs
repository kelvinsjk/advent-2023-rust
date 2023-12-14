//use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day14.txt").expect("Error reading input");
	// 	let input = "OOOO.#.O..
	// OO..#....#
	// OO..O##..O
	// O..#.OO...
	// ........#.
	// ..#....#.#
	// ..O..#.O.O
	// ..O.......
	// #....###..
	// #....#....";

	let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

	let mut cycles: usize = 0;
	let mut states = Vec::from([map_to_string(input.clone())]);
	let mut current = input.clone();

	let mut initial_number: Option<usize> = None;
	let mut cycle_number: Option<usize> = None;
	let mut target = 1000;
	let mut count = 0;

	while count < target {
		current = cycle(current.clone());
		let str = map_to_string(current.clone());
		cycles += 1;
		let score = count_load(&current);
		println!("Score {} at cycle {}", score, cycles);
		if initial_number.is_some() && cycle_number.is_some() {
			count += 1;
		//println!("Count to target: {}, {}, {}", count, target, cycles);
		} else {
			if states.contains(&str) {
				if initial_number.is_none() {
					initial_number = Some(cycles);
					println!("Initial: {}", cycles);
					states.clear();
				} else if cycle_number.is_none() {
					cycle_number = Some(cycles - initial_number.unwrap());
					let total = 1000000000;
					target = (total - initial_number.unwrap()) % cycle_number.unwrap();
					println!("Cycle: {}. Target: {}", cycle_number.unwrap(), target);
					states.clear();
				}
			}
		}
		states.push(str.clone());
	}

	// final result
	println!("Sum: {}", count_load(&current));
}

fn shift(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
	let row_len = input.len();
	let col_len = input[0].len();
	let mut output = vec![vec!['.'; col_len]; row_len];
	let mut maps: Vec<HashMap<i32, i32>> = Vec::new();
	let mut last_cube = vec![-1; col_len];
	let mut cubes: Vec<(usize, usize)> = Vec::new();
	for _ in 0..col_len {
		let map: HashMap<i32, i32> = HashMap::from([(-1, 0)]);
		maps.push(map);
	}
	// first iteration: get cubes and rocks
	for (row_idx, row) in input.iter().enumerate() {
		for (col_idx, char) in row.iter().enumerate() {
			match char {
				'.' => (),
				'O' => {
					let map: &mut HashMap<i32, i32> = maps.get_mut(col_idx).unwrap();
					let cube = map.entry(last_cube[col_idx]).or_insert(0).clone();
					map.insert(last_cube[col_idx], cube + 1);
				}
				'#' => {
					cubes.push((row_idx, col_idx));
					last_cube[col_idx] = row_idx as i32;
				}
				_ => panic!("Invalid char"),
			}
		}
	}
	//println!("{:#?}", cubes);
	// update output
	for (i, j) in cubes {
		output[i][j] = '#';
		let rocks = maps.get(j).unwrap().get(&(i as i32)).unwrap_or(&0).clone();
		for k in 0..rocks as usize {
			output[i + k + 1][j] = 'O';
		}
	}
	for j in 0..col_len {
		let rocks = maps.get(j).unwrap().get(&(-1)).unwrap_or(&0).clone();
		for k in 0..rocks as usize {
			output[k][j] = 'O';
		}
	}
	output
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
	assert!(!v.is_empty());
	let len = v[0].len();
	let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
	(0..len)
		.map(|_| {
			iters
				.iter_mut()
				.map(|n| n.next().unwrap())
				.collect::<Vec<T>>()
		})
		.collect()
}

// flip vertically
fn flip<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
	v.into_iter().rev().collect()
}

// flip horizontally
fn flip_x<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
	v.into_iter()
		.map(|n| n.into_iter().rev().collect())
		.collect()
}

fn cycle(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
	// north
	let mut cycle: Vec<Vec<char>> = shift(&input);
	// west
	cycle = transpose(cycle);
	cycle = shift(&cycle);
	cycle = transpose(cycle);
	// south
	cycle = flip(cycle);
	cycle = shift(&cycle);
	cycle = flip(cycle);
	// east
	cycle = flip_x(cycle);
	cycle = transpose(cycle);
	cycle = shift(&cycle);
	cycle = transpose(cycle);
	cycle = flip_x(cycle);
	cycle
}

fn map_to_string(v: Vec<Vec<char>>) -> String {
	let mut result = String::new();
	for row in v {
		for char in row {
			result.push(char);
		}
		result.push('\n');
	}
	result
}

fn count_load(input: &Vec<Vec<char>>) -> i32 {
	let mut sum = 0;
	let row_len = input.len() as i32;
	for (row_idx, row) in input.iter().enumerate() {
		for (_, char) in row.iter().enumerate() {
			match char {
				'O' => {
					sum += row_len - row_idx as i32;
				}
				_ => (),
			}
		}
	}
	sum
}
