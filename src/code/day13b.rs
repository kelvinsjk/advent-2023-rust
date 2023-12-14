//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day13.txt").expect("Error reading input");
	// 	let input = "#.##..##.
	// ..#.##.#.
	// ##......#
	// ##......#
	// ..#.##.#.
	// ..##..##.
	// #.#.##.#.

	// #...##..#
	// #....#..#
	// ..##..###
	// #####.##.
	// #####.##.
	// ..##..###
	// #....#..#";

	let mut sum = 0;
	let puzzles: Vec<&str> = input.split("\n\n").collect();
	let mut hits: Vec<Vec<usize>> = Vec::new();

	let puzzles_clone = puzzles.clone();
	for puzzle in puzzles {
		let mut hit: Vec<usize> = Vec::new();
		let mut lines: Vec<String> = Vec::new();
		let mut chars: Vec<Vec<char>> = Vec::new();
		let mut prev_line: &str = &"";
		let mut indices: Vec<usize> = Vec::new();

		for (i, line) in puzzle.lines().enumerate() {
			lines.push(line.to_string());
			if line == prev_line {
				indices.push(i);
			}
			prev_line = line;

			let line_chars = line.chars().collect::<Vec<char>>();
			chars.push(line_chars);
		}

		let row_len = lines.len();

		// check each row
		let mut row_sum = 0;
		for i in indices {
			let mut y = i.clone();
			let mut x = y - 1;
			loop {
				if x == 0 || y == row_len - 1 {
					row_sum += i;
					hit.push(i * 100);
					break;
				}
				x -= 1;
				y += 1;
				if lines[x] != lines[y] {
					break;
				}
			}
		}
		// println!("row sum: {:#?}", row_sum);
		sum += row_sum * 100;

		// transpose
		let col_len = lines[0].len();
		let mut row_iters: Vec<_> = chars.into_iter().map(Vec::into_iter).collect();
		let tranposed: Vec<Vec<char>> = (0..col_len)
			.map(|_| row_iters.iter_mut().map(|it| it.next().unwrap()).collect())
			.collect();
		lines = Vec::new();
		for line in tranposed {
			lines.push(line.iter().collect::<String>());
		}
		//println!("lines: {:#?}", lines);

		// get candidate indices
		indices = Vec::new();
		prev_line = &"";
		for (i, line) in lines.iter().enumerate() {
			if line == prev_line {
				indices.push(i);
			}
			prev_line = line;
		}
		// check each row
		let mut col_sum = 0;
		for i in indices {
			let mut y = i.clone();
			let mut x = y - 1;
			loop {
				if x == 0 || y == col_len - 1 {
					col_sum += i;
					hit.push(i);
					break;
				}
				x -= 1;
				y += 1;
				if lines[x] != lines[y] {
					break;
				}
			}
		}
		// println!("col sum: {:#?}", col_sum);
		sum += col_sum;
		hits.push(hit);
	}
	println!("Sum 1: {}", sum);

	sum = 0;

	for (p_idx, puzzle) in puzzles_clone.iter().enumerate() {
		let prev_hit = hits[p_idx].clone()[0];
		let row_len = puzzle.lines().count();
		let puzzle_len = puzzle.len();
		let col_len = (puzzle_len - (row_len - 1)) / row_len;

		'mutate: for mut_idx in 0..puzzle_len {
			if mut_idx % (col_len + 1) == col_len {
				continue;
			}
			let mut mutated_puzzle = puzzle.to_string().clone();
			let orig = mutated_puzzle.chars().nth(mut_idx).unwrap();
			let replacement = match orig {
				'.' => '#',
				'#' => '.',
				_ => {
					panic!("Invalid char")
				}
			};
			mutated_puzzle.replace_range(mut_idx..mut_idx + 1, &replacement.to_string());

			let mut lines: Vec<String> = Vec::new();
			let mut chars: Vec<Vec<char>> = Vec::new();
			let mut prev_line: &str = &"";
			let mut indices: Vec<usize> = Vec::new();

			for (i, line) in mutated_puzzle.lines().enumerate() {
				lines.push(line.to_string());
				if line == prev_line {
					indices.push(i);
				}
				prev_line = line;

				let line_chars = line.chars().collect::<Vec<char>>();
				chars.push(line_chars);
			}

			let row_len = lines.len();

			// check each row
			for i in indices {
				let mut y = i.clone();
				let mut x = y - 1;
				loop {
					if x == 0 || y == row_len - 1 {
						if i * 100 != prev_hit {
							sum += i * 100;
							break 'mutate;
						}
						break;
					}
					x -= 1;
					y += 1;
					if lines[x] != lines[y] {
						break;
					}
				}
			}

			// transpose
			let col_len = lines[0].len();
			let mut row_iters: Vec<_> = chars.into_iter().map(Vec::into_iter).collect();
			let tranposed: Vec<Vec<char>> = (0..col_len)
				.map(|_| row_iters.iter_mut().map(|it| it.next().unwrap()).collect())
				.collect();
			lines = Vec::new();
			for line in tranposed {
				lines.push(line.iter().collect::<String>());
			}
			//println!("lines: {:#?}", lines);

			// get candidate indices
			indices = Vec::new();
			prev_line = &"";
			for (i, line) in lines.iter().enumerate() {
				if line == prev_line {
					indices.push(i);
				}
				prev_line = line;
			}
			// check each row
			for i in indices {
				let mut y = i.clone();
				let mut x = y - 1;
				loop {
					if x == 0 || y == col_len - 1 {
						if i != prev_hit {
							sum += i;
							break 'mutate;
						}
						break;
					}
					x -= 1;
					y += 1;
					if lines[x] != lines[y] {
						break;
					}
				}
			}
		}
	}

	// final result
	println!("Sum 2: {}", sum);
	//println!("Sum: {:#?}", hits);
}
