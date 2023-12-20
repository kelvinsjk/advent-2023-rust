//use std::cmp;
use num::integer;
use std::collections::{HashMap, VecDeque};
use std::fs;

// needed the hint from
// ransoiong on reddit https://www.reddit.com/r/adventofcode/comments/18mmfxb/2023_day_20_solutions/
// to figure out to look for mutations

pub fn solution() {
	let input: String = fs::read_to_string("src/inputs/day20.txt").expect("Error reading input");

	let (broadcaster_outputs, mut node_map) = parse_input(&input);
	let og_signals: VecDeque<(&str, bool, &str)> = broadcaster_outputs
		.iter()
		.map(|&child| ("broadcaster", false, child))
		.collect();
	let mut signals = og_signals.clone();
	let mut button_pushes = 1;
	let mut cycles: Vec<usize> = Vec::new();
	let mut nodes = vec!["fv", "kk", "vt", "xr"];
	'signals: loop {
		if signals.len() == 0 {
			if button_pushes == 20000 {
				println!("terminated at {button_pushes}");
				break 'signals;
			}
			button_pushes += 1;
			signals = og_signals.clone();
			continue 'signals;
		}
		let (node_ref, signal, target_ref) = signals.pop_front().unwrap();
		let target = node_map.get_mut(target_ref);
		if target.is_none() {
			//println!("{} not found", target_ref);
			continue;
		}
		let target = target.unwrap();
		match target {
			Node::F(flip_flop) => {
				let output = flip_flop.receive(signal);
				match output {
					None => continue,
					Some(output) => {
						for child in flip_flop.children.iter() {
							signals.push_back((target_ref, output, child));
						}
					}
				}
			}
			Node::C(conjunction) => {
				let (output, changed) = conjunction.receive(node_ref, signal);
				for child in conjunction.children.iter() {
					signals.push_back((target_ref, output, child));
				}
				for (i, n) in nodes.clone().iter().enumerate() {
					if target_ref == *n && changed && button_pushes != 1 {
						let target = node_map.get_mut(n).unwrap();
						match target {
							Node::C(_) => {
								cycles.push(button_pushes);
								nodes.remove(i);
								if cycles.len() == 4 {
									let lcm = integer::lcm(
										integer::lcm(cycles[0], cycles[1]),
										integer::lcm(cycles[2], cycles[3]),
									);
									println!("lcm: {}", lcm);
									break 'signals;
								}
							}
							_ => (),
						}
					}
				}
			}
		}
		//for (_, node) in node_map.iter() {
		//	match node {
		//		Node::F(flip_flop) => {
		//			if flip_flop.on {
		//				continue 'signals;
		//			}
		//		}
		//		Node::C(conjunction) => {
		//			if !conjunction.all_low() {
		//				continue 'signals;
		//			}
		//		}
		//	}
		//}
		//while signals.len() > 0 {
		//	let (_, signal, _) = signals.pop_front().unwrap();
		//	if signal {
		//		high_pulses += 1;
		//	} else {
		//		low_pulses += 1;
		//	}
		//}
		//break 'signals;
	}

	println!("Button pushes: {}", button_pushes);
}

#[derive(Debug, Clone)]
struct FlipFlop<'a> {
	on: bool,
	children: Vec<&'a str>,
}

impl FlipFlop<'_> {
	// always sends low pulse
	fn receive(&mut self, signal: bool) -> Option<bool> {
		if signal {
			return None;
		}
		self.on = !self.on;
		if self.on {
			Some(true)
		} else {
			Some(false)
		}
	}
}

#[derive(Debug, Clone)]
struct Conjunction<'a> {
	memory: HashMap<&'a str, bool>, // low: false, high: true
	children: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
	fn receive(&mut self, parent: &'a str, signal: bool) -> (bool, bool) {
		let old_signal = self.memory.get(parent).unwrap();
		let changed = old_signal != &signal;
		self.memory.insert(parent, signal);
		// return low if all high.
		// return high if any low.
		for (_, high) in self.memory.iter() {
			if !high {
				return (true, changed);
			}
		}
		(false, changed)
	}

	// fn all_high(&self) -> bool {
	// 	for (_, high) in self.memory.iter() {
	// 		if !*high {
	// 			return false;
	// 		}
	// 	}
	// 	true
	// }

	// fn some_high(&self) -> bool {
	// 	for (_, high) in self.memory.iter() {
	// 		if *high {
	// 			return true;
	// 		}
	// 	}
	// 	false
	// }
}

#[derive(Debug, Clone)]
enum Node<'a> {
	F(FlipFlop<'a>),
	C(Conjunction<'a>),
}

// (broadcaster_outputs, node_map)
fn parse_input(input: &str) -> (Vec<&str>, HashMap<&str, Node>) {
	let mut broadcaster: Vec<&str> = Vec::new();
	let mut node_map: HashMap<&str, Node> = HashMap::new();
	for line in input.lines() {
		let (parent, children) = line.split_once(" -> ").unwrap();
		let children = children.split(", ");
		if parent == "broadcaster" {
			for child in children {
				broadcaster.push(child);
			}
			continue;
		}
		let parent_type = parent.chars().nth(0).unwrap();
		let parent = &parent[1..];
		match parent_type {
			'%' => {
				let mut flip_flop = FlipFlop {
					on: false,
					children: Vec::new(),
				};
				for child in children {
					flip_flop.children.push(child);
				}
				node_map.insert(parent, Node::F(flip_flop));
			}
			'&' => {
				let mut conjunction = Conjunction {
					memory: HashMap::new(),
					children: Vec::new(),
				};
				for child in children {
					conjunction.children.push(child);
				}
				node_map.insert(parent, Node::C(conjunction));
			}
			_ => panic!("Unexpected parent type: {}", parent_type),
		}
	}
	for (node_ref, node) in node_map.clone().into_iter() {
		let children = match node {
			Node::F(flip_flop) => flip_flop.children.clone(),
			Node::C(conjunction) => conjunction.children.clone(),
		};
		for child in children {
			let child_node = node_map.get_mut(child);
			if child_node.is_none() {
				//println!("{} not found", child);
				continue;
			}
			let child_node = child_node.unwrap();
			match child_node {
				Node::C(conjunction) => {
					conjunction.memory.insert(node_ref, false);
				}
				Node::F(_) => (),
			};
		}
	}
	//println!("{:#?}", node_map);
	(broadcaster, node_map)
}
