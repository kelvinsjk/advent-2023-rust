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
	let digits_strings = [
		"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];
	let mut sum = 0;
	for line in lines {
		//println!("Line: {line}");
		let mut first_digit: Option<u32> = None;
		let mut last_digit: Option<u32> = None;
		'iter_char: for (i, char) in line.chars().enumerate() {
			if first_digit.is_none() {
				if char.is_numeric() {
					first_digit = Some(char.to_digit(10).unwrap());
					if last_digit.is_some() {
						break 'iter_char;
					}
				} else {
					let rest_of_line = &line[i..];
					let mut j = 0;
					for digit in digits_strings {
						j += 1;
						if rest_of_line.starts_with(digit) {
							first_digit = Some(j);
							if last_digit.is_some() {
								break 'iter_char;
							} else {
								break; // digits_strings loop
							}
						}
					}
				}
			}
			if last_digit.is_none() {
				let char = line.chars().rev().nth(i).unwrap();
				if char.is_numeric() {
					last_digit = Some(char.to_digit(10).unwrap());
					if first_digit.is_some() {
						break 'iter_char;
					}
				} else {
					let rest_of_line = &line[0..line.chars().count() - i];
					let mut j = 0;
					for digit in digits_strings {
						j += 1;
						if rest_of_line.ends_with(digit) {
							last_digit = Some(j);
							if first_digit.is_some() {
								break 'iter_char;
							} else {
								break; // digits_strings loop
							}
						}
					}
				}
			}
		}
		if (first_digit.is_some()) && (last_digit.is_some()) {
			let first_digit = first_digit.unwrap();
			let last_digit = last_digit.unwrap();
			sum += first_digit * 10 + last_digit;
		} else {
			panic!("No digits found in line: {line}")
		}
	}
	println!("Sum: {}", sum);
}
