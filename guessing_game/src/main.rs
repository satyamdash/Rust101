use std::io;
fn main() {
    println!("Welcome to my guessing game");

    println!("Please input your guess.");

    let mut guess=String::new();

    io::stdin().read_line(&mut guess);

    println!("You guessed {}",guess);
}
