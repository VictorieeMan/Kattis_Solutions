//Created: 2023-10-03 by @VictorieeMan
//https://open.kattis.com/problems/tvaervikur
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;

fn ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

fn find_best_player_rank(player_id: usize, mut players: Vec<i32>) -> i32 {
	//Remove player_i from players
	let player_i = players.remove(player_id);

	//Sort players in descending order
	players.sort_by(|a, b| b.cmp(a));

	//find last standing player in players, to later challenge player_i
	//Strategy designed to weaken the strong players first
	let last_standing = loop {
		if players.len() <= 1 {
			if players.len() < 1 {
				break 0;
			}
			break players[0];
		}

		//Match up, two strongest players fight
		players[0] -= 1;
		players[1] -= 1;
		if players[0] <= 0 {
			if(players.len() == 2) {
				break players[0];
			}
			players.remove(0);
		}
		if players[1] <= 0 {
			players.remove(1);
		}

		//Sort players in descending order, rearrange if needed, strongest first!
		if players.len() > 2 && players[1] < players[2] {
			players.sort_by(|a, b| b.cmp(a));
		} else if players.len() == 2 && players[0] < players[1] {
			players.swap(0, 1);
		}
	};

	if(last_standing < player_i) {
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

	//Divide all values in hp_v by B, rounded up, to get strenght of each player
	for i in 0..n {
		hp_v[i as usize] = ceil_div(hp_v[i as usize], b);
	}

	for i in 0..n {
		//copy hp_v to hp_v_temp
		let players: Vec<i32> = hp_v.clone();
		//Find best rank for player i
		let best_rank = find_best_player_rank(i as usize, players);
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