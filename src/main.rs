use std::io;
use rand::Rng;

fn main() {
    println!("=== GUESS THE NUMBER ===");
    let secret_number= rand::thread_rng().gen_range(1..=100);
    println!("==> Secret number: {secret_number} <===");
    println!("Enter your guess: ");

    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid input!");

    println!("=== Your guess: {} ===", guess.trim());
}
