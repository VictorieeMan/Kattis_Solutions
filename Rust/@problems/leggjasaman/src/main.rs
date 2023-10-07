//Created: 2023-10-03
//https://open.kattis.com/problems/leggjasaman

use std::io;

fn main() {
    let stdin = io::stdin();

    // Read the first line as an integer
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    // Read the second line as an integer
    line = String::new();
    stdin.read_line(&mut line).unwrap();
    let m: i32 = line.trim().parse().unwrap();

    // Print the sum of the two integers
    println!("{}", n + m);
}
