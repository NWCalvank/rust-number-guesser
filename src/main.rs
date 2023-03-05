use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::io;

fn main() -> io::Result<()> {
    let answer: u32 = thread_rng().gen_range(1..11);
    let mut running = true;
    let mut user_auto = String::new();

    // TODO: Generate this range
    let mut guesses = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Automate? [y/N]");

    io::stdin().read_line(&mut user_auto)?;

    if user_auto.trim() == "y" {
        while running {
            // Pick a random number
            let machine_guess = guesses.choose(&mut thread_rng()).unwrap();
            println!("Guess: {}", machine_guess);

            // Check guess
            if machine_guess == &answer {
                running = false;
            } else {
                println!("Try again!");
            }

            // Sloppy clean-up
            let index = guesses.iter().position(|x| &*x == machine_guess).unwrap();
            guesses.remove(index);
        }
    } else {
        println!("Pick a number between 1 & 10 :)");

        while running {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            if user_input.trim() == answer.to_string() {
                running = false;
            } else {
                println!("Try again!");
            }
        }
    }

    println!("Correct!");

    Ok(())
}
