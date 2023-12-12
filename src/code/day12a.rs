//use std::cmp;
//use std::collections::HashMap;
use std::fs;

pub fn solution() {
	//let input = fs::read_to_string("src/inputs/sample07.txt").expect("Error reading input");
	let input = fs::read_to_string("src/inputs/day12.txt").expect("Error reading input");
	// 	let input = "???.### 1,1,3
	// .??..??...?##. 1,1,3
	// ?#?#?#?#?#?#?#? 1,3,1,6
	// ????.#...#... 4,1,1
	// ????.######..#####. 1,6,5
	// ?###???????? 3,2,1";

	let mut sum = 0;
	for line in input.lines() {
		//println!("Input: {}", line);
		let (map, reqs) = line.split_once(' ').expect("failed to split input row");
		let row: Vec<char> = map.chars().collect();
		let reqs: Vec<usize> = reqs
			.split(',')
			.map(|x| x.parse::<usize>().unwrap())
			.collect();
		//println!("Start. map: {:?}, reqs: {:?}", row, reqs);

		let ans = solutions(&row, &reqs);
		//println!("Ans: {}", ans);
		sum += ans;
	}

	// final result
	println!("Sum: {}", sum);
}

fn simplify(row: Vec<char>, reqs: Vec<usize>) -> Option<(Vec<char>, Vec<usize>)> {
	// if row starts/ends with # or ., the row can be simplified
	// returns simplified row and simplified reqs such that simplified row and reqs starts/ends with ?
	// None is returned if there is a contradiction: eg ##.?? for 3,1
	let mut row = row;
	let mut reqs = reqs;
	let mut first = row[0];
	while first == '#' || first == '.' {
		if first == '#' {
			if reqs.len() == 0 {
				return None;
			}
			let req = reqs.remove(0);
			if req > row.len() {
				return None;
			}
			let first_x = &row[0..req].to_vec();
			if (*first_x).contains(&'.') {
				return None;
			}
			let x = &row[req..];
			row = x.to_vec();
			if row.len() == 0 {
				return Some((row, reqs));
			}
			if row[0] == '#' {
				return None;
			} else if row[0] == '?' {
				row[0] = '.';
			}
		} else {
			let x = &row[1..];
			row = x.to_vec();
		}
		if row.len() == 0 {
			return Some((row, reqs));
		}
		first = row[0];
		continue;
	}
	// println!("Step1. map: {:?}, reqs: {:?}", row, reqs);
	let mut last = row[row.len() - 1];
	while last == '#' || last == '.' {
		if last == '#' {
			if reqs.len() == 0 {
				return None;
			}
			let req = reqs.pop().unwrap();
			let last_x = &row[row.len() - req..].to_vec();
			if (*last_x).contains(&'.') {
				return None;
			}
			let x = &row[..row.len() - req];
			row = x.to_vec();
			let row_len = row.len();
			if row_len == 0 {
				return Some((row, reqs));
			}
			if row[row_len - 1] == '#' {
				return None;
			} else if row[row_len - 1] == '?' {
				row[row_len - 1] = '.';
			}
		} else {
			let x = &row[..row.len() - 1];
			row = x.to_vec();
		}
		if row.len() == 0 {
			return Some((row, reqs));
		}
		last = row[row.len() - 1];
		continue;
	}
	// println!("Step2. map: {:?}, reqs: {:?}", row, reqs);
	Some((row, reqs))
}

fn solutions(row: &Vec<char>, reqs: &Vec<usize>) -> usize {
	let simplified = simplify(row.clone(), reqs.clone());
	let row: Vec<char>;
	let reqs: Vec<usize>;
	match simplified {
		Some((rw, rq)) => {
			row = rw;
			reqs = rq;
		}
		None => return 0,
	}
	if reqs.len() == 0 {
		if row.contains(&'#') {
			return 0;
		}
		return 1;
	}
	let sum = reqs.iter().sum::<usize>() + reqs.len() - 1;
	if row.len() < sum {
		return 0;
	} else if row.len() == sum {
		// create answer
		let mut ans: Vec<char> = Vec::new();
		for r in reqs {
			for _ in 0..r {
				ans.push('#');
			}
			ans.push('.');
		}
		ans.pop();
		for (i, a) in ans.iter().enumerate() {
			if (a == &'#' && row[i] == '.') || (a == &'.' && row[i] == '#') {
				return 0;
			}
		}
		return 1;
	}
	let mut new_row_hit = row.clone();
	new_row_hit[0] = '#';
	let mut new_row_miss = row.clone();
	new_row_miss[0] = '.';
	return solutions(&new_row_hit, &reqs) + solutions(&new_row_miss, &reqs);
}
