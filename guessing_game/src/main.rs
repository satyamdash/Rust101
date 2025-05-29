use std::io;
use rand::Rng;
fn main() {
    println!("Welcome to my guessing game");
    
    let secret_number = rand::rng().random_range(1..=100);
    println!("Please input your guess between 1 and 100.");

    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read input");

    println!("You guessed {}",guess);

    println!("The secret number is {}",secret_number);
}
