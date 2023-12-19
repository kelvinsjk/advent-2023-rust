//use std::cmp;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	let input: String = fs::read_to_string("src/inputs/day19a.txt").expect("Error reading input");
	// 	let input = "px{a<2006:qkq,m>2090:A,rfg}
	// pv{a>1716:R,A}
	// lnx{m>1548:A,A}
	// rfg{s<537:gd,x>2440:R,A}
	// qs{s>3448:A,lnx}
	// qkq{x<1416:A,crn}
	// crn{x>2662:A,R}
	// in{s<1351:px,qqz}
	// qqz{s>2770:qs,m<1801:hdj,R}
	// gd{a>3333:R,R}
	// hdj{m>838:A,pv}";
	let input = parse_input(&input);

	let mut ratings: Vec<Rating> = vec![Rating {
		ranges: [
			('x', (1, 4000)),
			('m', (1, 4000)),
			('s', (1, 4000)),
			('a', (1, 4000)),
		]
		.iter()
		.cloned()
		.collect(),
		workflow: "in",
		condition: 0,
	}];

	let mut sum = 0;

	while ratings.len() > 0 {
		let mut rating = ratings.pop().unwrap();
		'work: loop {
			match rating {
				Rating {
					workflow: "A",
					ranges,
					..
				} => {
					let (min_x, max_x) = ranges.get(&'x').unwrap();
					let (min_m, max_m) = ranges.get(&'m').unwrap();
					let (min_s, max_s) = ranges.get(&'s').unwrap();
					let (min_a, max_a) = ranges.get(&'a').unwrap();
					sum +=
						(max_x - min_x + 1) * (max_m - min_m + 1) * (max_s - min_s + 1) * (max_a - min_a + 1);
					break 'work;
				}
				Rating { workflow: "R", .. } => break 'work,
				_ => (),
			}
			let (conditions, outcomes) = input.get(&rating.workflow).unwrap();
			let condition = conditions.get(rating.condition).unwrap();
			let (stat, sign, val) = condition;
			let (min_val, max_val) = rating.ranges.get(stat).unwrap();
			match sign {
				'<' => {
					if max_val < val {
						// entire range pass
						rating.workflow = outcomes[rating.condition];
						rating.condition = 0;
						continue 'work;
					} else if min_val >= val {
						// entire range fail
						if rating.condition == conditions.len() - 1 {
							rating.workflow = outcomes[rating.condition + 1];
							rating.condition = 0;
						} else {
							rating.condition += 1;
						}
						continue 'work;
					} else {
						// range overlaps: split into pass and fail
						// get pass range and push
						let mut pass_range = rating.ranges.clone();
						pass_range.get_mut(stat).unwrap().1 = val - 1;
						ratings.push(Rating {
							ranges: pass_range,
							workflow: outcomes[rating.condition],
							condition: 0,
						});
						// continue with fail range
						rating.ranges.get_mut(stat).unwrap().0 = *val;
						if rating.condition == conditions.len() - 1 {
							rating.workflow = outcomes[rating.condition + 1];
							rating.condition = 0;
						} else {
							rating.condition += 1;
						}
						continue 'work;
					}
				}
				'>' => {
					if min_val > val {
						// entire range pass
						rating.workflow = outcomes[rating.condition];
						rating.condition = 0;
						continue 'work;
					} else if max_val <= val {
						// entire range fail
						if rating.condition == conditions.len() - 1 {
							rating.workflow = outcomes[rating.condition + 1];
							rating.condition = 0;
						} else {
							rating.condition += 1;
						}
						continue 'work;
					} else {
						// range overlaps: split into pass and fail
						// get pass range and push
						let mut pass_range = rating.ranges.clone();
						pass_range.get_mut(stat).unwrap().0 = val + 1;
						ratings.push(Rating {
							ranges: pass_range,
							workflow: outcomes[rating.condition],
							condition: 0,
						});
						// continue with fail range
						rating.ranges.get_mut(stat).unwrap().1 = *val;
						if rating.condition == conditions.len() - 1 {
							rating.workflow = outcomes[rating.condition + 1];
							rating.condition = 0;
						} else {
							rating.condition += 1;
						}
						continue 'work;
					}
				}
				_ => panic!("Invalid sign"),
			}
		}
	}

	println!("Solution: {}", sum);
}

#[derive(Debug, Clone)]
struct Rating<'a> {
	ranges: HashMap<char, (usize, usize)>,
	workflow: &'a str,
	condition: usize,
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
