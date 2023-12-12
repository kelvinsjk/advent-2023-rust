//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day09.txt").expect("Error reading input");
	// 	let input = "0 3 6 9 12 15
	// 1 3 6 10 15 21
	// 10 13 16 21 30 45";

	let mut sum: i64 = 0;

	for line in input.lines() {
		let seq: Vec<i64> = line
			.split_whitespace()
			.map(|x| x.parse::<i64>().expect("Error generating sequence"))
			.collect();
		let mut sequences = vec![seq];
		// create new sequences until we find an AP
		loop {
			let seq = sequences.last_mut().expect("Error getting last sequence");
			let predicted = predict_next(&seq);
			match predicted {
				Some(val) => {
					seq.push(val);
					break;
				}
				None => {
					sequences.push(generate_new_seq(
						sequences.last().expect("Error getting last sequence"),
					));
				}
			}
		}
		while sequences.len() > 1 {
			let final_val = &sequences
				.pop()
				.unwrap()
				.last()
				.expect("Error getting last value")
				.clone();
			let seq = sequences.last_mut().expect("Error getting last sequence");
			seq.push(seq.last().expect("Error getting last value") + final_val);
		}
		// println!("Sequence: {:?}", sequences[0].last());
		sum += sequences[0].last().expect("Error getting last value");
	}

	// final result
	println!("Sum: {}", sum);
}

// warning: tests only last value
fn predict_next(seq: &Vec<i64>) -> Option<i64> {
	let diff = seq[1] - seq[0];
	let n = seq.len();
	let predicted = seq[0] + (n as i64 - 1) * diff;
	if predicted == seq[n - 1] {
		Some(predicted + diff)
	} else {
		None
	}
}

fn generate_new_seq(seq: &Vec<i64>) -> Vec<i64> {
	let mut new_vec: Vec<i64> = Vec::new();
	for i in 0..(seq.len() - 1) {
		new_vec.push(seq[i + 1] - seq[i]);
	}
	new_vec
}
