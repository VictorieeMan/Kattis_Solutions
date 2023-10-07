//Created: 2023-10-07 by @VictorieeMan
//https://open.kattis.com/problems/guessinggame
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;

fn check_if_in_bounds(guess:i32,lowest:i32,highest:i32)->bool{
	if guess < lowest {
		return false;
	}
	if highest < guess {
		return false;
	}
	return true;
}

fn main() {
	let stdin = io::stdin();

	//Guesses for values of n in [1,10] and feedback on each guess.
	let mut guess: i32;
	let mut feedback: Vec<String>;
	let mut active_tournament:bool = true; //Tournament is the set of games.
	let mut tournament_results:Vec<&str> = Vec::new(); //For the output strings.

	while active_tournament { //Tournament loop!
		let mut active_game:bool = true; //New game! Var, used for flow control.
		let mut highest:i32 = 10; //Upper bound of n
		let mut lowest:i32 = 1; //Lower bound of n
		let mut honesty:bool = true; //Stan is assumed to be honest <3
		'game_loop: loop { //Game loop!
			//Dealing with input
			let mut input_str: String = String::new();
			stdin.read_line(&mut input_str)
				.expect("Failed to read guess line.");

			guess = input_str.trim().parse::<i32>().unwrap();
			if guess == 0 {
				active_tournament = false;
				break; //Terminate tournament, no more i/o.
			}

			input_str.clear();
			stdin.read_line(&mut input_str)
				.expect("Failed to read feedback line.");

			let trimmed_str = input_str.trim().to_string().clone();
			feedback = trimmed_str.split_whitespace().map(str::to_owned).collect();

			//Checking the second word in the feedback
			/*
			If Stan is determined as dishonest, we set active_game=0 and
			ignore to evaluate more of his nonsense. New trust next round.
			The reason we can't just break is that we still need to handle input
			until next game starts or the tournament ends.
			*/
			if active_game && feedback[1] == "high"{//Guess is to high
				if guess < lowest {
					//If feedback is high, but guess is smaller than lowest bound.
					active_game = false;
					honesty = false;
				}
				highest = std::cmp::min(highest, guess - 1);
				if highest < lowest{
					//ex. when h=3 and l=5, then there is no number in between that can be correct.
					active_game = false;
					honesty = false;
				}
			} 
			
			else if active_game && feedback[1] == "low"{//Guess is to low
				if guess > highest {
					//If feedback is low, but guess is smaller than highest bound.
					active_game = false;
					honesty = false;
				}
				lowest = std::cmp::max(lowest, guess + 1);
				if lowest > highest{
					//ex. when h=3 and l=5, then there is no number in between that can be correct.
					active_game = false;
					honesty = false;
				}
			} 
			
			else if feedback[0] == "right" { //Assume "right on"
				//Right on ends the current game, but altering the active_game bool isn't needed.
				if !honesty {
					if !check_if_in_bounds(guess, lowest, highest) {
						//If the correct answer was outside of the feedback bounds.
						// println!("Stan is dishonest");
						honesty = false;
						break 'game_loop;
					} else {
						honesty = true;
						break 'game_loop;
					}
				} else {
					break 'game_loop;
				}
			}
		};

		if !active_tournament{
			//If not active, then terminate the program.
			break;
		}

		if honesty {
			tournament_results.push("Stan may be honest");
		} else if !honesty {
			tournament_results.push("Stan is dishonest");
		} else {
			break;
		}
	};

	for result in tournament_results {
		println!("{}", result);
	}
}