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
	let seeds_len = seeds.len();

	for stage in 1..8 {
		//let input = fs::read_to_string(format!("src/inputs/day05/sample_{}.txt", stage))
		let input = fs::read_to_string(format!("src/inputs/day05/map_{}.txt", stage))
			.expect("Error reading input");
		'seed_iter: for i in 0..seeds_len {
			let seed = seeds[i];
			for line in input.lines() {
				let [dest, src, len] = line
					.split_whitespace()
					.take(3)
					.map(|n| n.parse::<u64>().expect("Error parsing seeds"))
					.collect::<Vec<u64>>()[..]
				else {
					panic!("Error parsing line")
				};
				if seed >= src && seed < src + len {
					seeds[i] = dest + (seed - src);
					continue 'seed_iter;
				}
			}
		}
		println!("Stage {stage}: {:?}", seeds);
	}

	// final result
	let min = seeds.iter().min().expect("Error finding min");
	println!("Lowest: {}", min);
}
