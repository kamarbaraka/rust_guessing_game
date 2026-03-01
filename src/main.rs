use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=== GUESS THE NUMBER ===");

    loop {
        let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
        //println!("==> Secret number: {secret_number} <===");
        println!("Enter your guess: (1-100)");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Invalid input!");

        guess.retain(|c| !c.is_whitespace());

        if guess == "quit" {println!("===> goodbye!");break;}

        let guess: u8 = match guess.parse() {
            Ok(num) => num,
            Err(err) => {println!("=!= {}", err);continue;}
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("===< Too small ==="),
            Ordering::Greater => println!("===> Too big ==="),
            Ordering::Equal => {
                println!("=== You WIN ===");
                break;
            }
        }

        println!("=== Your guess: {} ===", guess);
    }
}
