use std::io;
use rand::prelude::*;

fn main() {
    // Prompt user for input
    println!("Guess the number!");
    println!("Please input your guess.");

    // Setup mutable variable to store user input then store it
    let mut guess = String::new(); // Is this the same as var = "" in other languages
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);


    // Print out the secret and user input
    println!("The secret number is: {secret_number}");
    println!("You guessed: {}", guess);
}
