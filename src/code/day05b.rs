//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/day05/sample_seeds.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day05/seeds.txt").expect("Error reading input");
	let mut seeds: Vec<u64> = input
		.split_whitespace()
		.map(|n| n.parse::<u64>().expect("Error parsing seeds"))
		.collect();

	for stage in 1..8 {
		let seeds_len = seeds.len();
		//let input = fs::read_to_string(format!("src/inputs/day05/sample_{}.txt", stage))
		let input = fs::read_to_string(format!("src/inputs/day05/map_{}.txt", stage))
			.expect("Error reading input");
		let mut new_seeds: Vec<u64> = Vec::new();
		for i in 0..(seeds_len / 2) {
			let seed = seeds[2 * i];
			let len_seed = seeds[2 * i + 1];
			new_seeds = [new_seeds, update_range(seed, len_seed, input.clone())].concat();
		}
		seeds = new_seeds;
		//println!("Stage {stage}: {:?}", seeds);
	}

	// final result
	let seeds = seeds
		.iter()
		.enumerate()
		.filter(|(i, _)| i % 2 == 0)
		.map(|(_, n)| n)
		.collect::<Vec<&u64>>();
	let min = seeds.iter().min().expect("Error finding min");
	println!("Lowest: {}", min);
}

fn update_range(start: u64, len_seed: u64, input: String) -> Vec<u64> {
	let mut new_range: Vec<u64> = Vec::new();
	let line_len = input.lines().count();
	let mut line_no = 0;
	for line in input.lines() {
		let [dest, src, len] = line
			.split_whitespace()
			.take(3)
			.map(|n| n.parse::<u64>().expect("Error parsing seeds"))
			.collect::<Vec<u64>>()[..]
		else {
			panic!("Error parsing line")
		};
		if start >= src && start < src + len {
			new_range.push(dest + (start - src));
			if start + len_seed <= src + len {
				new_range.push(len_seed);
				break;
			} else {
				let new_len = src + len - start;
				new_range.push(new_len);
				// final number in range is src + len - 1
				// starting seed is start
				// length of this new range is
				// src + len - 1 - start + 1
				// = src + len - start
				new_range = [
					new_range,
					update_range(src + len, len_seed - new_len, input.clone()),
				]
				.concat();
				break;
				// length in new_range is src + len - start
				// orig len is len_seed
				// so remaining len is
				// len_seed - (src + len - start)
			}
		}
		line_no += 1;
		if line_no == line_len {
			new_range.push(start);
			new_range.push(len_seed);
		}
	}
	new_range
}
