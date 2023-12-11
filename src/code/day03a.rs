//use std::cmp;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample03.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day03.txt").expect("Error reading input");
	let mut sum = 0;
	let lines: Vec<&str> = input.split("\n").collect();
	let rows = lines.len();
	let mut row = 0;
	for line in &lines {
		let cols = line.chars().count();
		let mut number_mode = false;
		let mut number_start = 0;
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
					if is_part(row, rows, number_start, end, cols, line, &lines) {
						sum += number;
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
	println!("Sum: {}", sum);
}

fn is_symbol(char: char) -> bool {
	!(char.is_numeric() || char == '.')
}

fn is_part(
	row: usize,
	rows: usize,
	start: usize,
	end: usize,
	cols: usize,
	line: &str,
	lines: &Vec<&str>,
) -> bool {
	let left = match start {
		0 => 0,
		_ => start - 1,
	};
	let right = match end {
		_ if end == cols - 1 => end + 1,
		_ => end + 2,
	};
	let prev_row_hit = match row {
		0 => false,
		_ => lines.get(row - 1).expect("Failed to get previous line")[left..right].contains(is_symbol),
	};
	let next_row_hit = match row {
		_ if row == rows - 1 => false,
		_ => lines.get(row + 1).expect("Failed to get next line")[left..right].contains(is_symbol),
	};
	let curr_row_hit_left = match start {
		0 => false,
		_ => is_symbol(
			line
				.chars()
				.nth(start - 1)
				.expect("Failed to get left char"),
		),
	};
	let curr_row_hit_right = is_symbol(line.chars().nth(end + 1).unwrap_or('.'));
	prev_row_hit || next_row_hit || curr_row_hit_left || curr_row_hit_right
}
