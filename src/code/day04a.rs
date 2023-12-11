//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample04.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day04.txt").expect("Error reading input");
	let mut sum = 0;
	let lines: Vec<&str> = input.split("\n").collect();
	for line in &lines {
		let (_, card) = line.split_once(": ").expect("Error reading card");
		let (winning_numbers, card_numbers) = card.split_once(" | ").expect("Error splitting card");
		let winning_numbers: Vec<u32> = winning_numbers
			.split_whitespace()
			.map(|n| n.parse::<u32>().expect("Error parsing winning number"))
			.collect();
		let card_numbers: Vec<u32> = card_numbers
			.split_whitespace()
			.map(|n| n.parse::<u32>().expect("Error parsing card number"))
			.collect();
		let mut score = 0;
		for number in winning_numbers {
			if card_numbers.contains(&number) {
				score = match score {
					0 => 1,
					_ => score * 2,
				}
			}
		}
		sum += score;
	}
	// final result
	println!("Sum: {}", sum);
}
