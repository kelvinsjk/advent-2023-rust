//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day11.txt").expect("Error reading input");
	// 	let input = "...#......
	// .......#..
	// #.........
	// ..........
	// ......#...
	// .#........
	// .........#
	// ..........
	// .......#..
	// #...#.....";

	let mut sum = 0;
	let mut map: Vec<Vec<char>> = Vec::new();
	let mut col_expand_arr: Vec<usize> = Vec::new();
	let mut row_expand_arr: Vec<usize> = Vec::new();

	// set up map, get indices of empty rows and cols
	for (i, line) in input.lines().enumerate() {
		let line_chars = line.chars().collect::<Vec<char>>();
		map.push(line_chars.clone());
		// initialize col
		if i == 0 {
			col_expand_arr = (0..line.len()).collect();
		}
		// whittle down cols to expand
		let mut new_col_expand_arr: Vec<usize> = Vec::new();
		for col in col_expand_arr {
			if line_chars[col] == '.' {
				new_col_expand_arr.push(col);
			}
		}
		col_expand_arr = new_col_expand_arr;
		// check if row needs to expand
		if !line_chars.contains(&'#') {
			row_expand_arr.push(i);
		}
	}

	let mut galaxies: Vec<(usize, usize)> = Vec::new();
	for (row, line) in map.iter().enumerate() {
		for (col, c) in line.iter().enumerate() {
			if *c == '#' {
				galaxies.push((row, col));
			}
		}
	}

	const MULTIPLE: u64 = 1_000_000;
	for (i, galaxy) in galaxies.iter().enumerate() {
		for other_galaxy in &galaxies[i + 1..] {
			let mut xs = [galaxy.0, other_galaxy.0];
			xs.sort();
			let [left, right] = xs;
			let mut ys = [galaxy.1, other_galaxy.1];
			ys.sort();
			let [top, bottom] = ys;
			let mut col_betweens = 0;
			let mut row_betweens = 0;
			for row in &row_expand_arr {
				if row > &left && row < &right {
					row_betweens += 1;
				}
			}
			for col in &col_expand_arr {
				if col > &top && col < &bottom {
					col_betweens += 1;
				}
			}
			let x_distance = row_betweens * MULTIPLE + (right - left - row_betweens as usize) as u64;
			let y_distance = col_betweens * MULTIPLE + (bottom - top - col_betweens as usize) as u64;

			sum += x_distance + y_distance;
		}
	}

	// final result
	println!("Sum: {}", sum);
}
