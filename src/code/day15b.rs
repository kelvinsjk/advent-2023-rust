//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day15.txt").expect("Error reading input");
	//let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
	let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];

	let mut sum = 0;
	for row in input.split(',') {
		if row.contains('-') {
			let (label, _) = row.split_once('-').unwrap();
			let box_index = hash_number(&label);
			let bx = boxes.get_mut(box_index as usize).unwrap();
			let index = bx.iter().position(|x| x.0 == label);
			if let Some(index) = index {
				bx.remove(index);
				println!(
					"{} removed at box {}. left with {:#?}",
					label, box_index, bx
				);
			} else {
				println!("nothing removed");
			}
		} else if row.contains('=') {
			let (label, value) = row.split_once('=').unwrap();
			let box_index = hash_number(&label);
			let bx = boxes.get_mut(box_index as usize).unwrap();
			let index = bx.iter().position(|x| x.0 == label);
			if let Some(index) = index {
				bx.splice(
					index..index + 1,
					vec![(label, value.parse::<u8>().unwrap())],
				);
				println!(
					"{} updated at box {}. left with {:#?}",
					label, box_index, bx
				);
			} else {
				bx.push((label, value.parse::<u8>().unwrap()));
				println!("{} added at box {}. left with {:#?}", label, box_index, bx);
			}
		} else {
			println!("{}", row);
			panic!("Invalid input")
		}
	}

	for (box_number, bx) in boxes.iter().enumerate() {
		for (slot_index, (_, value)) in bx.iter().enumerate() {
			let power = (box_number + 1) * (slot_index + 1) * (*value as usize);
			sum += power;
		}
	}

	// final result
	println!("Sum: {}", sum);
}

fn hash_number(str: &str) -> u32 {
	let mut sum = 0;
	for char in str.chars() {
		let val = char.to_string().as_bytes()[0] as u32;
		sum += val;
		sum *= 17;
		sum = sum % 256;
	}
	sum
}
