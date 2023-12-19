//use std::cmp;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	let input: String = fs::read_to_string("src/inputs/day19a.txt").expect("Error reading input");
	let input = parse_input(&input);
	let parts = parse_parts();

	let mut sum = 0;
	for part in parts {
		let mut workflow = "in";
		'work: loop {
			//println!("{}", workflow);
			if workflow == "A" {
				sum += part.get(&'*').unwrap();
				break 'work;
			}
			if workflow == "R" {
				break 'work;
			}
			let (conditions, outcomes) = input.get(workflow).unwrap();
			for (i, (stat, sign, val)) in conditions.iter().enumerate() {
				let part_val = part.get(stat).unwrap();
				match sign {
					'<' => {
						if part_val < val {
							workflow = outcomes[i];
							break;
						}
					}
					'>' => {
						if part_val > val {
							workflow = outcomes[i];
							break;
						}
					}
					_ => panic!("Unexpected sign: {}", sign),
				}
				workflow = outcomes[conditions.len()];
			}
		}
	}
	println!("{}", sum);
}

fn parse_input(input: &str) -> HashMap<&str, (Vec<(char, char, usize)>, Vec<&str>)> {
	let mut map: HashMap<&str, (Vec<(char, char, usize)>, Vec<&str>)> = HashMap::new();
	for line in input.lines() {
		let mut conditions: Vec<(char, char, usize)> = Vec::new();
		let mut outcomes: Vec<&str> = Vec::new();
		let (key, condition_str) = line.split_once('{').unwrap();
		let rules = condition_str.split(",");
		let len = rules.clone().collect::<Vec<&str>>().len();
		for (i, rule) in rules.enumerate() {
			if i == len - 1 {
				outcomes.push(&rule[..rule.len() - 1]);
				break;
			}
			let (rule, outcome) = rule.split_once(":").unwrap();
			conditions.push((
				rule.chars().nth(0).unwrap(),
				rule.chars().nth(1).unwrap(),
				rule[2..].parse::<usize>().unwrap(),
			));
			outcomes.push(outcome);
		}
		map.insert(key, (conditions, outcomes));
	}
	map
}

fn parse_parts() -> Vec<HashMap<char, usize>> {
	// 	let input = "{x=787,m=2655,a=1222,s=2876}
	// {x=1679,m=44,a=2067,s=496}
	// {x=2036,m=264,a=79,s=2244}
	// {x=2461,m=1339,a=466,s=291}
	// {x=2127,m=1623,a=2188,s=1013}";
	let input = fs::read_to_string("src/inputs/day19b.txt").expect("Error reading input");
	let mut vec: Vec<HashMap<char, usize>> = Vec::new();
	for line in input.lines() {
		let mut map: HashMap<char, usize> = HashMap::new();
		let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
		let caps = re.captures(line).unwrap();
		let x = caps[1].parse::<usize>().unwrap();
		let m = caps[2].parse::<usize>().unwrap();
		let a = caps[3].parse::<usize>().unwrap();
		let s = caps[4].parse::<usize>().unwrap();
		// sum
		map.insert('*', x + m + a + s);
		map.insert('x', x);
		map.insert('m', m);
		map.insert('a', a);
		map.insert('s', s);
		vec.push(map);
	}
	vec
}
