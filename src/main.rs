use std::io;

fn main() {
    println!("=== GUESS THE NUMBER ===");
    println!("Enter the nuber: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess.trim()).expect("Invalid input!");

    println!("=== Your guess: {guess} ===");
}
