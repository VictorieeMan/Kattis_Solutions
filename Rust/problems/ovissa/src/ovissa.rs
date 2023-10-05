//Created: 2023-10-05 by @VictorieeMan
//https://open.kattis.com/problems/ovissa
//Repository URL: https://github.com/VictorieeMan/Kattis_Solutions

use std::io;

fn main() {
	//Reading string from input
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	let u_count = input.matches("u").count();

	//Calculating length of string
	println!("{}", u_count);
}

// fn main(){let mut i=String::new();std::io::stdin().read_line(&mut i);let u=i.matches("u").count();println!("{}",u);}
// fn main(){let mut i=String::new();std::io::stdin().read_line(&mut i);println!("{}",i.matches('u').count())}
// fn main(){let mut i=String::new();std::io::stdin().read_line(&mut i);print!("{}",i.matches('u').count())}
