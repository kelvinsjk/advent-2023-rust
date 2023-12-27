//use std::cmp;
//use std::collections::HashMap;

pub fn solution() {
	let pos1: [i64; 3] = [326286404372227, 109008617563040, 380856240762368];
	let pos2: [i64; 3] = [489391903974223, 224519329306789, 464487706304279];
	let pos3: [i64; 3] = [188222403544810, 318242021601895, 221953095005789];
	let d1 = [-96, 287, -193];
	let d2 = [-202, 132, -126];
	let d3 = [157, -11, 152];
	//let pos1 = [19, 13, 30];
	//let pos2 = [18, 19, 22];
	//let pos3 = [12, 31, 28];
	//let d1 = [-2, 1, -2];
	//let d2 = [-1, -1, -2];
	//let d3 = [-1, -2, -1];

	let min = 500; //1;
	let max = 1000;

	'a: for sum in min..max {
		println!("sum {}", sum);
		for i in 0..sum {
			for j in 0..sum - i {
				let k = sum - i - j;
				if i == 0 && j == 0 && k == 0 {
					continue;
				}
				let d_arrays: [[i64; 3]; 8] = [
					[i, j, k],
					[i, j, -k],
					[i, -j, k],
					[i, -j, -k],
					[-i, j, k],
					[-i, j, -k],
					[-i, -j, k],
					[-i, -j, -k],
				];
				'b: for d in &d_arrays {
					let row1 = vec![1, 0, 0, d[0] - d1[0], 0, pos1[0]];
					let row2 = vec![0, 1, 0, d[1] - d1[1], 0, pos1[1]];
					let row3 = vec![0, 0, 1, d[2] - d1[2], 0, pos1[2]];
					let row4 = vec![1, 0, 0, 0, d[0] - d2[0], pos2[0]];
					//let row5 = vec![0, 1, 0, 0, d[1] - d2[1], pos2[1]];
					let row6 = vec![0, 0, 1, 0, d[2] - d2[2], pos2[2]];

					let matrix = vec![row1, row2, row3, row4, row6];
					let solutions = gaussian_elimination(matrix);
					if solutions.is_none() {
						continue;
					}
					let solutions = solutions.unwrap();
					if solutions.len() == 0 {
						println!("infinite solution");
						continue;
					}
					let t2 = solutions[0];
					let t1 = solutions[1];
					let z = solutions[2];
					let y = solutions[3];
					let x = solutions[4];
					//println!("t1: {}, t2: {}, x: {}, y: {}, z: {}", t1, t2, x, y, z);
					if t1 < 0 || t2 < 0 {
						//println!("negative t");
						continue;
					}
					let lhs = z + t2 * (d[2] - d2[2]);
					if lhs != pos2[2] {
						//println!("z not equal");
						continue;
					}
					let x0 = [x, y, z];
					let den = d[0] - d3[0];
					if den == 0 {
						//println!("den == 0");
						continue;
					}
					let num = pos3[0] - x0[0];
					if num % den != 0 {
						//println!("no integer solution");
						continue;
					}
					let t3 = num / den;
					if t3 < 0 {
						//println!("negative t");
						continue;
					}
					for i in 1..3 {
						let lhs = x0[i] + t3 * (d[i] - d3[i]);
						if lhs != pos3[i] {
							//println!("{} not equal", i);
							continue 'b;
						}
					}
					println!("Final answer: {}, {:?}", x + y + z, d);
					println!("Final answer: ({},{},{}), {:?}, {},{}", x, y, z, d, t1, t2);
					break 'a;
				}
			}
		}
	}
}

fn gaussian_elimination(mut matrix: Vec<Vec<i64>>) -> Option<Vec<i64>> {
	for i in 0..matrix.len() {
		let mut first = matrix[i][i];
		if first == 0 {
			if i == matrix.len() - 1 {
				if matrix[i][matrix.len()] == 0 {
					return Some(Vec::new());
				}
				return None;
			}
			for new_i in i + 1..matrix.len() {
				let new_first = matrix[new_i][i];
				if new_first != 0 {
					let old_row = matrix[i].clone();
					for j in 0..matrix.len() + 1 {
						matrix[i][j] += matrix[new_i][j];
						matrix[new_i][j] = old_row[j];
					}
					first = matrix[i][i];
					break;
				} else if new_i == matrix.len() - 1 {
					//println!("No solution found at row {}", i);
					return None;
				}
			}
		}
		for j in i + 1..matrix.len() {
			let second = matrix[j][i];
			let divisor = gcd(first, second);
			let multiple1 = second / divisor;
			let multiple2 = first / divisor;
			for k in 0..matrix.len() + 1 {
				matrix[j][k] *= multiple2;
				matrix[j][k] -= matrix[i][k] * multiple1;
			}
		}
	}
	let mut solutions: Vec<i64> = Vec::new();
	for i in (0..matrix.len()).rev() {
		let mut num = matrix[i][matrix.len()];
		for j in i + 1..matrix.len() {
			num -= matrix[i][j] * solutions[matrix.len() - 1 - j];
		}
		if matrix[i][i] == 0 {
			if num == 0 {
				return Some(Vec::new());
			}
			return None;
		}
		if num % matrix[i][i] != 0 {
			// no integer solution
			return None;
		}
		solutions.push(num / matrix[i][i]);
	}
	Some(solutions)
}

fn gcd(mut n: i64, mut m: i64) -> i64 {
	n = n.abs();
	m = m.abs();
	if n == 0 {
		return m;
	}
	if m == 0 {
		return n;
	}
	while m != 0 {
		if m < n {
			std::mem::swap(&mut m, &mut n);
		}
		m %= n;
	}
	n
}
