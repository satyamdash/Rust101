use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to my guessing game");
    
    let secret_number = rand::rng().random_range(1..=100);

    loop {
    println!("Please input your guess between 1 and 100.");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }

    }

}
