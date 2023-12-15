//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day15.txt").expect("Error reading input");
	//let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

	let mut sum = 0;
	for row in input.split(',') {
		let mut inner_sum = 0;
		for char in row.chars() {
			let val = char.to_string().as_bytes()[0] as u32;
			inner_sum += val;
			inner_sum *= 17;
			inner_sum = inner_sum % 256;
		}
		println!("{}: {}", sum, inner_sum);
		sum += inner_sum;
	}

	// final result
	println!("Sum: {}", sum);
}
