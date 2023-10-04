//Created: 2023-10-03 by @VictorieeMan
//https://open.kattis.com/problems/tvaervikur
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;
use std::mem::swap;
use std::collections::BinaryHeap;

fn ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

fn quick_check(player_i: i32, players_len: i32, s_max: i32, s_2nd_max: i32) -> bool {
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

	//Assigning c_max to be the strongest challenger, not player_i
	let mut c_max = if player_i == s_max {s_2nd_max} else {s_max};

	//Checking against conjecture 3
	let numb_of_challengers = players_len - 1;
	if player_i >= c_max+1 {
		return true;
	} else if (player_i >= c_max-1 && numb_of_challengers % 2 == 0) {
		return true;
	} 
	return false;
}

fn find_best_player_rank(player_id: usize, players: &Vec<i32>, s_max: i32, s_2nd_max: i32) -> i32 {
    /*
    Function returns the best rank for player_i, as of conjecture 1, player_i
    is guaranteed to be among the last two players. Hence either a 1 or a two is
    returned. THe guick_check() function uses a heuristic to determine if
    player_i won rank 1, if it returns 0, then further analysis is needed.
    */
	let players_len = players.len() as i32;
    ////////////////////
    //If only one player
    if players_len == 1 {
        return 1;
    }

    /////////////////////////
    //If more than one player
    //Extract player_i from players
    let player_i = players[player_id];

    /////////////////////
    //If only two players
    if players_len == 2 {
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
	if players_len == 3 {
		//Logics for assigning c1 and c2, depending on player_id.
		//id numbers must be in range 0..2
		let c1_id = if player_id == 0 {1} else {0};
		let c2_id;
		if c1_id == 0{
			c2_id = if player_id == 1 {2} else {1};
		} else {// c1_id == 1;
			c2_id = if player_id == 2 {0} else {2};
		}
		let mut c1;
		let mut c2;
		if players[c1_id] > players[c2_id]{
			c1 = players[c1_id];
			c2 = players[c2_id];
		} else {
			c1 = players[c2_id];
			c2 = players[c1_id];
		}

		//Let c1 and c2 figth each other, till one is left standing.
		c1 -= c2;
		c2 = 0;

		let challenger = c1;
		if player_i >= challenger {
			return 1;
		} else {
			return 2;
		}
	}

	////////////////////////////
	//If more than three players

    //Quick check if player_i wins rank 1
    if quick_check(player_i, players_len, s_max, s_2nd_max) {
        // No more investigation needed, player_i wins rank 1; exit.
        return 1;
    }

	//Seperate the challengers from the player_i
	let mut challengers = Vec::with_capacity(players_len as usize - 1);
	for i in 0..players_len as usize {
		if i != player_id {
			challengers.push(players[i]);
		}
	}

	//Sort challengers in descending order
	challengers.sort_by(|a, b| b.cmp(a));

	//Fight among challengers, strongest first
	let last_standing = loop {

		let mut i = 0;
		while i + 1 < challengers.len() {
			challengers[i] -= challengers[i+1];
			
			//Challenger = 0, remove from list
			challengers.remove(i+1);

			if challengers[i] <= 0 {
            challengers.remove(i);
			} else {
				i += 1;
			}
		}

		if challengers.len() == 1 {
			break challengers[0];
		} else if challengers.len() == 0 {
			break  0;
		}
	};

	//Fight last_standing against player_i
    if player_i >= last_standing {
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
    let mut players: Vec<i32> = Vec::with_capacity(n as usize);
	let mut s_max = -1;
	let mut s_2nd_max = -1; //Second largest in list
    for i in 0..n as usize {
        //Divide all values in hp_v by B, rounded up, to get strenght of each player
        let strength = ceil_div(iter.next().unwrap().parse().unwrap(), b);
        players.push(strength);
		if strength > s_max {
			s_2nd_max = s_max;
			s_max = strength;
		}
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
    let mut max_rankings: Vec<i32> = vec![-1; n as usize];  //max over all games

    for i in 0..n {
        //Find best rank for player i
        let best_rank = find_best_player_rank(i as usize, &players, s_max, s_2nd_max);
		//Directly print the best rank
		print!("{} ", best_rank);
    }
}