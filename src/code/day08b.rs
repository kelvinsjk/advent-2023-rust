//use std::cmp;
use num::integer::lcm;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day08.txt").expect("Error reading input");
	// 	let input = "11A = (11B, XXX)
	// 11B = (XXX, 11Z)
	// 11Z = (11B, XXX)
	// 22A = (22B, XXX)
	// 22B = (22C, 22C)
	// 22C = (22Z, 22Z)
	// 22Z = (22B, 22B)
	// XXX = (XXX, XXX)";

	// handle map
	let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
	for line in input.lines() {
		let (key, dirs) = line.split_once(" = (").expect("Error reading line");
		let (left, right) = dirs.split_once(", ").expect("Error reading directions");
		let right = &right[..3];
		map.insert(key, [left, right]);
	}

	// handle directions
	//let directions = "LR";
	let directions = "LRRLRRRLRRRLLRRLRRLRLRLRRLLRRLRRLRRRLLLRRRLRRRLRRRLLRRRLRRLLRRLRRLRLRRRLRRLRLRRLRRRLLRRLLRLRRRLLRRLRRLLLRLRRRLRLRLRLLRRRLRLLRRRLRLRRRLRRRLLRRLRRRLLRRLRLLRLRRLLLRRLRRLLLRLLRLRRRLRLRLRRRLRRLLRRRLRLRLRRLRRRLRLRRLRRLRRRLRRRLRRRLRRRLRRLLRRLRLLRRLLRRRLRLLRLRLRRLRRLRLRLRRRLRLRLRRLRLRRLRRRR";
	let directions = directions.chars().collect::<Vec<char>>();
	let dir_len = directions.len();

	// handle starting locations
	let mut locs: Vec<&str> = Vec::new();
	for (key, _) in &map {
		if key.ends_with("A") {
			locs.push(key);
		}
	}
	println!("Starting locations: {:?}", locs);

	// start loop
	let mut lcm_val: u64 = 1;
	for loc_start in locs {
		let mut step = 0;
		let mut moves = 0;
		let mut loc = loc_start;
		while !loc.ends_with("Z") {
			let [left, right] = map.get(&loc).expect("Error reading map directions");
			let dir = directions[step];
			loc = match dir {
				'L' => *left,
				'R' => *right,
				_ => panic!("Error getting next location"),
			};
			step = (step + 1) % dir_len;
			moves += 1;
		}
		lcm_val = lcm(lcm_val, moves);
	}

	// final result
	println!("Moves: {}", lcm_val);
}
