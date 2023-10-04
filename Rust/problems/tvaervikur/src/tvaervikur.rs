//Created: 2023-10-03 by @VictorieeMan
//https://open.kattis.com/problems/tvaervikur
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

fn quick_check(player_i: i32, players: &Vec<i32>) -> bool {
	//player_i, is mentioned as s_i in the conjectures below.
	/*
	Conjecture 3: When player s_i waits out the other challengers to fight
		among them selves, the pigeon principle leads to the conclusion that
		there are two guaranteed wins for s_i depending on ewather there are
		an even or odd numbers of challengers left. These are:
		*If even number of challengers: s_i >= c_max-1
		*If odd number of challengers: s_i >= c_max+1
		Where c_max is the strength of the strongest challenger.
	
	This function will check these as a heuristic and return true if s_i wins
	rank 1, and false if s_i loses to the challenger => gets rank 2.

	If this function evaluates to 1, the anser is final.
	Else if the function evaluates to 0, the answer is not final, and further
	analysis is needed to determine the final rank of s_i.
	*/

	//Sort players in descending order
	let mut players_sorted = players.to_vec();
	players_sorted.sort_by(|a, b| b.cmp(a));

	let c_max = players_sorted[0];
	if player_i >= c_max+1 {
		return true;
	} else if (player_i >= c_max-1 && players_sorted.len() % 2 == 0) {
		return true;
	} 
	return false;
}

fn find_best_player_rank(player_id: usize, players: &Vec<i32>) -> i32 {
	/*
	Function returns the best rank for player_i, as of conjecture 1, player_i
	is guaranteed to be among the last two players. Hence either a 1 or a two is
	returned. THe guick_check() function uses a heuristic to determine if
	player_i won rank 1, if it returns 0, then further analysis is needed.
	*/
	////////////////////
	//If only one player
	if players.len() == 1 {
		return 1;
	}

	/////////////////////////
	//If more than one player
	//Extract player_i from players
	let player_i = players[player_id];

	/////////////////////
	//If only two players
	if players.len() == 2 {
		if player_i > players[1] {
			return 1;
		} else {
			return 2;
		}
	}

	//////////////////////////
	//If more than two players

	///////////////////////
	//If only three players
	if players.len() == 3 {
		//Extract the two challenger players, c1 and c2
		for i in 1..players.len() {
			if i != player_id {
				let c1 = players[i];
				let c2 = players[(i+1)%3];

				//Let c1 and c2 figth each other, till one is left
				let challenger = (c1-c2).abs();
				if challenger < player_i {
					return 1;
				} else {
					return 2;
				}
			}
		}
	}

	////////////////////////////
	//If more than three players

	//Quick check if player_i wins rank 1
	if quick_check(player_i, &players) {
		// No more investigation needed, player_i wins rank 1; exit.
		return 1;
	}

	//Add the rest of the players to a max heap. (these are the challengers)
	//Rust heaps are max by default.
	let mut challenger_heap: BinaryHeap<_> = players.iter().enumerate()
		.filter(|&(idx, _)| idx != player_id)
		.map(|(_, &player)| player)
		.collect();

	//find last standing player in players, to later challenge player_i
	//Strategy designed to weaken the strong players first

	//Extract top 3 players, use third as a peak forward.
	let mut top1 = challenger_heap.pop().unwrap();
	let mut top2 = challenger_heap.pop().unwrap();
	let mut top3 = challenger_heap.pop().unwrap();
	let mut more_than_two: bool = true;
	let mut first_run: bool = true;

	while first_run || challenger_heap.len() > 1{
		// while top2 > 0 && top1 > 0{
		// 	top1 -= 1;
		// 	top2 -= 1;
		// }
		//Equivalent to the while loop above, but faster
		top1 -= top2;
		top2 = 0;

		if more_than_two && top2 < top3 {
			//Indicates that it's time to extract a new top3
			if top1 > 0 {
				challenger_heap.push(top1);
			}
			if top2 > 0 {
				challenger_heap.push(top2);
			}
			if top3 > 0 {
				challenger_heap.push(top3);
			}
			more_than_two = if challenger_heap.len() > 2 {true} else {false};

			if challenger_heap.len() > 1{
				//Extract new top3
				top1 = challenger_heap.pop().unwrap();
				top2 = challenger_heap.pop().unwrap();
				if more_than_two {
					top3 = challenger_heap.pop().unwrap();
				}
			}
		}

		if more_than_two != true && (top1 == 0 || top2 == 0){
			if top1 > 0 {
				challenger_heap.push(top1);
			}
			if top2 > 0 {
				challenger_heap.push(top2);
			}
			break;
		}
	}

	//Testing out some (to me) new rust assignment syntax
	let last_standing = 
	if challenger_heap.is_empty() {
		0
	} else {
		challenger_heap.pop().unwrap()
	};

	if(last_standing <= player_i) {
		return 1;
	} else {
		return 2;
	}
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

	//Parsing to integer vector players
	// players is vector of Hp for each contestant
	let mut iter = line.split_whitespace();
	let mut players: Vec<i32> = Vec::new();
	for _ in 0..n {
		//Divide all values in hp_v by B, rounded up, to get strenght of each player
		let strength = ceil_div(iter.next().unwrap().parse().unwrap(), b);
		players.push(strength);
	}

	/* Problem formulation:
	There are n contestants, each with players[i] hp. They are identified by their
	index i in hp_v. At every moment there are still alive = n-dead contestants
	left.Each time two collide in a fight, they lose B hp each. When a contest-
	ant's hp reaches 0, they are eliminated and removed from the game:
	dead++, alive-- and player i dies at rank = alive+1.

	We want to iterate through all combinations of possible games, and find the
	highest achievable rank of each player i. We store the highest rank of each
	player in a vector rank_v, where index i corresponds to player i.
	Then we print this vector.
	*/

	/*
	Matematical reasoning and simplification of problem:
	Let S be an unordered set of n players, where each player i has hp_v[i] hp.
	We have s_i = ceil(hp_v[i]/B) strength of each player i, where strength is
	the number of fights a player can survive. We want to find the highest rank.

	*Conjecture 1: If s_i, waits for the other to fight among them selves until
		the very end, then s_i will be guaranteed to be among the last two.
		*s_i rank 1: if challenger is weaker.
		*s_i rank 2: if challenger is equal or stronger, due to the rules.
		if true => We need to find the strenght of the challenger, and compare.
		Conjecture 2: The weakest challenger is derived by only letting the
			strongest players fight each other until the end game.
	
	These are unproven, but derived from some napkin math and intuition.
	A prototype will now be implemented, and tested on the sample input.
	*/

	// init rank_v with -1, indicating not yet calculated
	let mut max_rankings: Vec<i32> = vec![-1; n as usize];	//max over all games

	for i in 0..n {
		//Find best rank for player i
		let best_rank = find_best_player_rank(i as usize, &players);
		//Update max_rankings
		// if best_rank > max_rankings[i as usize] {
			max_rankings[i as usize] = best_rank;
		// }
	}

	//Print max_rankings
	for i in 0..n {
		print!("{} ", max_rankings[i as usize]);
	}
}