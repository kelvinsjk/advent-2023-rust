//use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample03.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day03.txt").expect("Error reading input");
	let mut sum = 0;
	let mut star_index_to_numbers_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
	let lines: Vec<&str> = input.split("\n").collect();
	let rows = lines.len();
	let mut row = 0;
	for line in &lines {
		let cols = line.chars().count();
		let mut number_mode = false;
		let mut number_start = 0;
		// loop through lines to get *-indices and adjacent numbers map
		for (col, char) in line.chars().enumerate() {
			if number_mode {
				// searching for end of number
				if !char.is_numeric() || col == cols - 1 {
					number_mode = false;
					let end = match col {
						_ if col == cols - 1 && char.is_numeric() => col,
						_ => col - 1,
					};
					let number = &line[number_start..end + 1]
						.parse::<u32>()
						.expect("Error parsing number");
					println!("{number}");
					let star_indices = get_star_indices(row, rows, number_start, end, cols, line, &lines);
					for star_index in star_indices {
						let star_index_vec = star_index_to_numbers_map
							.entry(star_index)
							.or_insert(Vec::new());
						star_index_vec.push(*number);
					}
				}
			} else {
				// searching for new number
				if char.is_numeric() {
					number_mode = true;
					number_start = col;
				}
			}
		}
		row += 1;
	}
	// loop through map to find sums
	println!("{star_index_to_numbers_map:#?}");
	for (_, numbers) in star_index_to_numbers_map {
		if numbers.len() == 2 {
			sum += numbers[0] * numbers[1];
		}
	}
	// final result
	println!("Sum: {}", sum);
}

fn get_star_indices(
	row: usize,
	rows: usize,
	start: usize,
	end: usize,
	cols: usize,
	line: &str,
	lines: &Vec<&str>,
) -> Vec<(usize, usize)> {
	let mut star_indices: Vec<(usize, usize)> = Vec::new();
	// left
	if start != 0
		&& line
			.chars()
			.nth(start - 1)
			.expect("Error getting left char")
			== '*'
	{
		star_indices.push((row, start - 1));
	}
	// right
	if end != cols - 1 && line.chars().nth(end + 1).expect("Error getting right char") == '*' {
		star_indices.push((row, end + 1));
	}
	// set up prev/next line
	let left = match start {
		0 => 0,
		_ => start - 1,
	};
	let right = match end {
		_ if end == cols - 1 => end + 1,
		_ => end + 2,
	};
	// loop through prev line
	if row != 0 {
		let prev_line = lines.get(row - 1).expect("Failed to get previous line");
		for (col, char) in prev_line[left..right].chars().enumerate() {
			if char == '*' {
				star_indices.push((row - 1, left + col));
			}
		}
	}
	// loop through next line
	if row != rows - 1 {
		let next_line = lines.get(row + 1).expect("Failed to get next line");
		for (col, char) in next_line[left..right].chars().enumerate() {
			if char == '*' {
				star_indices.push((row + 1, left + col));
			}
		}
	}
	star_indices
}
