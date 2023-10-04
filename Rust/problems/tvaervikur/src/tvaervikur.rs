//Created: 2023-10-03 by @VictorieeMan
//https://open.kattis.com/problems/tvaervikur
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;
use std::mem::swap;
use std::collections::BinaryHeap;

fn ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

fn quick_check(player_i: i32, players_sorted: &mut Vec<i32>) -> bool {
	//player_i, is mentioned as s_i in the conjectures below.
	//Function assumes that players_sorted is sorted in descending order.
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

	// It's important to not double count player_i, hence we remove one value,
	// equal to player_i. Specific removal of player_i is equivalent.
	// Using bin-search to find the position of a player_i value in the vector.

	let mut pos = match players_sorted.binary_search(&player_i) {
		Ok(index) => index as i32,
		Err(_) => -1,
	};

	// Check if pos is last index
	let last_idx = players_sorted.len() - 1;
	let pos_is_last: bool = (pos == last_idx as i32);

	//Now we use the pos value to ignore player_i in the following analysis.
	let c_max = if pos_is_last {
		players_sorted[last_idx - 1] // If player_i is last, then c_max is the second last
	} else {
		players_sorted[last_idx] // Else c_max is the last
	};

	//Checking against conjecture 3
	let numb_of_challengers = last_idx;
	if player_i >= c_max+1 {
		return true;
	} else if (player_i >= c_max-1 && numb_of_challengers % 2 == 0) {
		return true;
	} 
	return false;
}

fn find_best_player_rank(player_id: usize, players: &Vec<i32>, players_sorted: &mut Vec<i32>) -> i32 {
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
    if quick_check(player_i, players_sorted) {
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

    while true {
		//Simulating the two strongest fighting till one is left standing.
        top1 -= top2;
        top2 = 0;

		if top3 > top1 {
			//Swap top1 and top3
			swap(&mut top1, &mut top3);
			if top3 > 0 {
                challenger_heap.push(top3);
            }
		}

		more_than_two = if challenger_heap.len() > 2 {true} else {false};

		if !challenger_heap.is_empty() {
			//Extract new top2 and top3
			top2 = challenger_heap.pop().unwrap();
			if more_than_two {
				top3 = challenger_heap.pop().unwrap();
			}
		}

        if more_than_two == false && (top1 == 0 || top2 == 0){
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

    if(player_i >= last_standing) {
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
    for i in 0..n as usize {
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
    let mut max_rankings: Vec<i32> = vec![-1; n as usize];  //max over all games

	//Sort players in descending order
	let mut players_sorted = players.to_vec();
	players_sorted.sort();

    for i in 0..n {
        //Find best rank for player i
        let best_rank = find_best_player_rank(i as usize, &players, &mut players_sorted);
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