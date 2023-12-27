//use std::cmp;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph;
use std::collections::HashMap;
use std::fs;

pub fn solution() {
	// 	let input = "jqt: rhn xhk nvd
	// rsh: frs pzl lsr
	// xhk: hfx
	// cmg: qnr nvd lhk bvb
	// rhn: xhk bvb hfx
	// bvb: xhk hfx
	// pzl: lsr hfx nvd
	// qnr: nvd
	// ntq: jqt hfx bvb xhk
	// nvd: lhk
	// lsr: lhk
	// rzs: qnr cmg lsr rsh
	// frs: qnr lhk lsr";
	let input: String = fs::read_to_string("src/inputs/day25.txt").expect("Error reading input");
	let (graph, nodes) = parse_input(&input);
	println!(
		"{:?}, {}, {}",
		graph.node_count(),
		graph.edge_count(),
		nodes.len()
	);

	let (_, partition) = stoer_wagner_min_cut(&graph, |_| Ok::<usize, usize>(1))
		.unwrap()
		.unwrap();
	let cut2 = nodes.len() - partition.len();
	let ans = partition.len() * cut2;
	println!("min_cut: {:?}", ans);
}

fn parse_input(
	input: &str,
) -> (
	petgraph::graph::UnGraph<&str, usize>,
	HashMap<&str, petgraph::graph::NodeIndex>,
) {
	let mut nodes: HashMap<&str, petgraph::graph::NodeIndex> = HashMap::new();
	let mut graph = petgraph::graph::UnGraph::<&str, usize>::new_undirected();
	let mut visited: Vec<&str> = Vec::new();
	for line in input.lines() {
		let (u, vs) = line.split_once(": ").unwrap();
		if !visited.contains(&u) {
			visited.push(u);
			nodes.insert(u, graph.add_node(u));
		}
		let vs: Vec<&str> = vs.split(' ').collect();
		for v in vs {
			if !visited.contains(&v) {
				visited.push(v);
				nodes.insert(v, graph.add_node(v));
			}
		}
	}
	for line in input.lines() {
		let (u, vs) = line.split_once(": ").unwrap();
		let u_node = nodes.get(u).unwrap();
		let vs: Vec<&str> = vs.split(' ').collect();
		for v in vs {
			let v_node = nodes.get(v).unwrap();
			graph.add_edge(*u_node, *v_node, 1);
		}
	}
	(graph, nodes)
}
