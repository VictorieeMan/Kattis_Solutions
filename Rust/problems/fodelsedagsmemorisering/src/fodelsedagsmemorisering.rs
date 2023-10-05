//Created: 2023-10-05 by @VictorieeMan
//https://open.kattis.com/problems/fodelsedagsmemorisering
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io::{self, BufRead};
use std::collections::HashMap;
fn main() {
	let stdin = io::stdin();

	let mut input_line = String::new();
	stdin.read_line(&mut input_line).unwrap();

	// n is the number of people that will be listed next
	let n: usize = input_line.trim().parse::<usize>().unwrap();

	input_line.clear();

	let mut adress_book: HashMap<String, (String, i32)> = HashMap::new();
	for line in stdin.lock().lines().map(|l| l.unwrap()).take(n) {
		let line_columns = line.split_whitespace().collect::<Vec<&str>>();
		let name = line_columns[0].to_string();
		let score = line_columns[1].parse::<i32>().unwrap();
		let date = line_columns[2].to_string();

		let person = (name, score);

		if adress_book.contains_key(&date) {
			let (_, existing_score) = adress_book.get(&date).unwrap();
			if score > *existing_score {
				adress_book.insert(date, person);
			}
		} else {
			adress_book.insert(date, person);
		}
	}

	let mut adress_book_vec: Vec<(String, (String, i32))> = adress_book.into_iter().collect();
	adress_book_vec.sort_by(|a, b| a.1.0.cmp(&b.1.0));

	println!("{}", adress_book_vec.len());
	for (_, (name, _)) in adress_book_vec {
		println!("{}", name);
	}
}