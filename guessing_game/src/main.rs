use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generating a randm number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    // println!("Please input your guess.");

    loop {
        // Creating a new string for the user input
        let mut guess = String::new();

        // Listening for the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting that input into an int - Ignoring the wrong inputs (anything that is not a
        // number)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Returning a string depending on the comparison
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
