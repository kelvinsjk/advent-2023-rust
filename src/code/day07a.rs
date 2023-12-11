use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day07.txt").expect("Error reading input");

	let mut vec: Vec<([u8; 6], u32)> = Vec::new();
	for line in input.lines() {
		let (card, bid) = line.split_once(" ").expect("Error reading line");
		let bid = bid.parse::<u32>().expect("Error parsing bid");
		vec.push((score(card), bid));
	}

	vec.sort();

	let mut winnings = 0;
	for (i, (_, bid)) in vec.iter().enumerate() {
		winnings += (i + 1) as u32 * bid;
	}

	// final result
	println!("Winnings: {}", winnings);
}

fn format_input(input: &str) -> [char; 5] {
	let mut chars = input.chars();
	[
		chars.next().unwrap(),
		chars.next().unwrap(),
		chars.next().unwrap(),
		chars.next().unwrap(),
		chars.next().unwrap(),
	]
}

fn input_to_map(chars: [char; 5]) -> HashMap<char, u8> {
	let mut map: HashMap<char, u8> = HashMap::new();
	for char in chars {
		let count = map.entry(char).or_insert(0);
		*count += 1;
	}
	map
}

fn char_score(char: char) -> u8 {
	match char {
		'A' => 14,
		'K' => 13,
		'Q' => 12,
		'J' => 11,
		'T' => 10,
		_ => char.to_digit(10).expect("failed to parse char") as u8,
	}
}

fn score(input: &str) -> [u8; 6] {
	let chars = format_input(input);
	let map = input_to_map(chars);
	let overall_score: u8 = match map.len() {
		5 => 1, // high card
		4 => 2, // one pair
		3 => {
			let mut max_pair = 0;
			for (_, count) in map {
				max_pair = cmp::max(max_pair, count);
			}
			match max_pair {
				2 => 3, // two pairs
				3 => 4, // three of a kind
				_ => panic!("Unexpected count for map of length 3"),
			}
		}
		2 => {
			let mut max_pair = 0;
			for (_, count) in map {
				max_pair = cmp::max(max_pair, count);
			}
			match max_pair {
				3 => 5, // full house
				4 => 6, // four of a kind
				_ => panic!("Unexpected count for map of length 3"),
			}
		}
		1 => 7, // five of a kind
		_ => panic!("Unexpected map length"),
	};
	let char_scores = chars.map(|c| char_score(c));
	[
		overall_score,
		char_scores[0],
		char_scores[1],
		char_scores[2],
		char_scores[3],
		char_scores[4],
	]
}
