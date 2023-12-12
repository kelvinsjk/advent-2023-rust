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

	// set up map, expand if necessary
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
	let mut idx = 0;
	for col in col_expand_arr {
		for row in 0..map.len() {
			map[row].insert(col + idx, '.');
		}
		idx += 1;
	}
	idx = 0;
	for row in row_expand_arr {
		let new_row = vec!['.'; map[0].len()];
		map.insert(row + idx, new_row);
		idx += 1;
	}

	let mut galaxies: Vec<(usize, usize)> = Vec::new();
	for (row, line) in map.iter().enumerate() {
		for (col, c) in line.iter().enumerate() {
			if *c == '#' {
				galaxies.push((row, col));
			}
		}
	}

	for (i, galaxy) in galaxies.iter().enumerate() {
		for other_galaxy in &galaxies[i + 1..] {
			let distance = ((galaxy.0 as i64) - (other_galaxy.0 as i64)).abs()
				+ ((galaxy.1 as i64) - (other_galaxy.1 as i64)).abs();
			sum += distance;
		}
	}

	// final result
	println!("Sum: {}", sum);
}
