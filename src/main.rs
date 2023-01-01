use rand::Rng;
use std::io;
fn main() {
    for i in 0..5 {
        let arr = ["R", "P", "S"];
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..arr.len());

        let c = rng.gen_range(0..arr.len());
        println!("This is a Game of Rock, Paper,  Scissor");

        println!("Player 1 ");
        let mut player1 = arr[c];
        println!("{}", arr[c]);
        
        println!("Player 2 ");
        let mut player2 = arr[n];
        println!("{}", arr[n]);

        if player1 == "R" && player2 == "S" {
            println!("Player 1 wins");
        } else if player1 == "R" && player2 == "P" {
            println!("player 2 wins");
        } else if player1 == "R" && player2 == "R" {
            println!("TIE");
        } else if player1 == "S" && player2 == "R" {
            println!("Player 2 Wins");
        } else if player1 == "S" && player2 == "P" {
            println!("Player 1 Wins");
        } else if player1 == "S" && player2 == "S" {
            println!("TIE");
        } else if player1 == "P" && player2 == "R" {
            println!("Player 1 Wins");
        } else if player1 == "P" && player2 == "S" {
            println!("Player 2 Wins");
        } else if player1 == "P" && player2 == "P" {
            println!("TIE");
        }
    }
}
