use std::cmp;
use std::fs;

struct MaxCubes {
	blue: u32,
	red: u32,
	green: u32,
}

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample02.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day02.txt").expect("Error reading input");
	let lines: Vec<&str> = input.split("\n").collect();
	let mut sum = 0;
	for line in lines {
		let mut max_cubes = MaxCubes {
			blue: 0,
			red: 0,
			green: 0,
		};
		let sets = line.split(": ").nth(1).expect("Error getting cubes");
		let subsets: Vec<&str> = sets.split("; ").collect();
		for cubes in subsets {
			let colors: Vec<&str> = cubes.split(", ").collect();
			for color_str in colors {
				let (number, color) = color_str.split_once(" ").expect("Error getting numbers");
				let number = number.parse::<u32>().expect("Error parsing number");
				match color {
					"blue" => max_cubes.blue = cmp::max(number, max_cubes.blue),
					"red" => max_cubes.red = cmp::max(number, max_cubes.red),
					"green" => max_cubes.green = cmp::max(number, max_cubes.green),
					_ => println!("Unknown color {} obtained", color),
				}
			}
		}
		sum += max_cubes.blue * max_cubes.red * max_cubes.green;
	}
	println!("Sum: {}", sum);
}
