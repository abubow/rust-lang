use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    if guess.trim() == "quit" {
        println!("Quitting");
        return;
    }
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    // check if guess is correct
    if guess == secret_number {
        println!("You guessed correctly!");
    } else {
        println!("You lose!");
    }
}
