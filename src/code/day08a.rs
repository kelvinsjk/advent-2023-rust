//use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day08.txt").expect("Error reading input");
	// 	let input = "AAA = (BBB, CCC)
	// BBB = (DDD, EEE)
	// CCC = (ZZZ, GGG)
	// DDD = (DDD, DDD)
	// EEE = (EEE, EEE)
	// GGG = (GGG, GGG)
	// ZZZ = (ZZZ, ZZZ)";
	// 	let input = "AAA = (BBB, BBB)
	// BBB = (AAA, ZZZ)
	// ZZZ = (ZZZ, ZZZ)";

	// handle map
	let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
	for line in input.lines() {
		let (key, dirs) = line.split_once(" = (").expect("Error reading line");
		let (left, right) = dirs.split_once(", ").expect("Error reading directions");
		let right = &right[..3];
		map.insert(key, [left, right]);
	}

	// handle directions
	//let directions = "RL";
	//let directions = "LLR";
	let directions = "LRRLRRRLRRRLLRRLRRLRLRLRRLLRRLRRLRRRLLLRRRLRRRLRRRLLRRRLRRLLRRLRRLRLRRRLRRLRLRRLRRRLLRRLLRLRRRLLRRLRRLLLRLRRRLRLRLRLLRRRLRLLRRRLRLRRRLRRRLLRRLRRRLLRRLRLLRLRRLLLRRLRRLLLRLLRLRRRLRLRLRRRLRRLLRRRLRLRLRRLRRRLRLRRLRRLRRRLRRRLRRRLRRRLRRLLRRLRLLRRLLRRRLRLLRLRLRRLRRLRLRLRRRLRLRLRRLRLRRLRRRR";
	let directions = directions.chars().collect::<Vec<char>>();
	let dir_len = directions.len();

	let mut loc = "AAA";
	let mut moves = 0;
	let mut step = 0;

	while loc != "ZZZ" {
		let [left, right] = map.get(&loc).expect("Error reading map directions");
		let dir = directions[step];
		match dir {
			'L' => loc = *left,
			'R' => loc = *right,
			_ => panic!("Error getting next location"),
		}
		moves += 1;
		step = (step + 1) % dir_len;
	}

	// final result
	println!("Moves: {}", moves);
}
