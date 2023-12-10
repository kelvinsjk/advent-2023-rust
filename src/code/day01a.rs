use std::fs;

pub fn solution() {
	let read = fs::read_to_string("src/inputs/day01.txt");
	let mut input = String::new();
	match read {
		Ok(i) => {
			input = i;
		}
		Err(e) => println!("Error reading input: {e}"),
	}
	let lines: Vec<&str> = input.split("\n").collect();
	let mut sum = 0;
	for line in lines {
		println!("Line: {line}");
		let mut number = String::new();
		for c in line.chars() {
			if c.is_numeric() {
				number.push(c);
				break;
			}
		}
		for c in line.chars().rev() {
			if c.is_numeric() {
				number.push(c);
				break;
			}
		}
		match number.parse::<u32>() {
			Ok(n) => sum += n,
			Err(e) => println!("Error parsing number: {e}. Number: {number}"),
		}
	}
	println!("Sum: {}", sum);
}
