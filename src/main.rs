use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret numner is {secret_number}");

    loop {
        println!("Please enter your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // TODO: maybe instead of expect, handle the error and not crash?
        let guess: u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(hi) => hi,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
