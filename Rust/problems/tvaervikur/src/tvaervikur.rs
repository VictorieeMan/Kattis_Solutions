//Created: 2023-10-03 by @VictorieeMan
//https://open.kattis.com/problems/tvaervikur
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;

fn simulate_game(initial_hp: &Vec<i32>, encounters: &[(i32, i32)], b: i32) -> Vec<i32> {
    // simulation logic here

	for &(i, j) in encounters.iter() {
		// Update HP and check for eliminations
	}
	
}


fn generate_encounters(n: i32) -> Vec<(i32, i32)> {
    // generation logic here
}

fn ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

fn main() {
	let stdin = io::stdin();

	// Reading the first line
	let mut line = String::new();
	stdin.read_line(&mut line).unwrap();

	//Parsing to integers n and B
	// n is number of contestants
	// B is Hp damage from encounter
	let mut iter = line.split_whitespace();
	let n: i32 = iter.next().unwrap().parse().unwrap();
	let b: i32 = iter.next().unwrap().parse().unwrap();

	// Reading the second line
	line = String::new();
	stdin.read_line(&mut line).unwrap();

	//Parsing to integer vector hp_v
	// hp_v is vector of Hp for each contestant
	let mut iter = line.split_whitespace();
	let mut hp_v: Vec<i32> = Vec::new();
	for _ in 0..n {
		hp_v.push(iter.next().unwrap().parse().unwrap());
	}

	/* Problem formulation:
	There are n contestants, each with hp_v[i] hp. They are identified by their
	index i in hp_v. At every moment there are still alive = n-dead contestants
	left.Each time two collide in a fight, they lose B hp each. When a contest-
	ant's hp reaches 0, they are eliminated and removed from the game:
	dead++, alive-- and player i dies at rank = alive+1.

	We want to iterate through all combinations of possible games, and find the
	highest achievable rank of each player i. We store the highest rank of each
	player in a vector rank_v, where index i corresponds to player i.
	Then we print this vector.
	*/

	// init rank_v with -1, indicating not yet calculated
	let mut rankings: Vec<i32> = vec![-1; initial_hp.len()]; //per game
	let mut max_rankings: Vec<i32> = vec![-1; n as usize];	//max over all games

	// Generating all possible encounters
	let encounters = generate_encounters(n);

	for encounters in generate_encounters(n) {
		let rankings = simulate_game(&hp_v, &encounters, b);
		// update highest rank achieved for each player
	}

	// Was about to move forward with simulating all games, but that might be a combinatorial explosion.
	// New idea is to use the metric max_encounters = ceil(h_i/B) for each player i, and then try ro individually find the best winnign path for eah player; in games where the other contestans make the best moves fro player i rather than themselves.

}