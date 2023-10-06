//Created: 2023-10-06 by @VictorieeMan
//https://open.kattis.com/problems/fluortanten
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

#[inline]
fn scalar_multiplication(a: &[i64], b: &[i64]) -> i64 {
	let mut result: i64 = 0;
	for i in 0..a.len() {
		result += a[i] * b[i];
	}
	result
}

fn main() {
	let stdin = std::io::stdin();
	let mut input = String::new();

	// Reding first line and parsing it to n: usize
	stdin.read_line(&mut input).unwrap();
	let n: usize = input.trim().parse().unwrap();
	input.clear();

	// Reading second line and parsing it to a vector of u32
	stdin.read_line(&mut input).unwrap();

	let mut line_of_quers: Vec<i64> = Vec::with_capacity(n);
	for a in input.trim().split_whitespace() {
		line_of_quers.push(a.parse::<i64>().unwrap());
	}
	input.clear();

	// Remove value equal to 0; equivalent to moving Björn to the end of the line.
	line_of_quers.retain(|&x| x != 0);
	let que_len = n - 1; // Excluding Björn

	let que_pos: Vec<i64> = (1..=que_len as i64).collect(); // Vector of que positions
	let mut que_sum = scalar_multiplication(&que_pos, &line_of_quers);
	let mut max_result = que_sum;

	for i in 0..que_len {
		let a_i = line_of_quers[que_len - 1 - i];
		que_sum += a_i;

		max_result = std::cmp::max(max_result, que_sum);
	}

	println!("{}", max_result);

}