use rand::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut len = 10;
    let mut running = true;
    let mut user_len = String::new();
    let mut user_auto = String::new();

    println!("Between 1 and ...? [(10)/100/1000]");
    io::stdin().read_line(&mut user_len)?;

    if user_len.trim() == "100" {
        len = 100
    }

    if user_len.trim() == "1000" {
        len = 1000
    }

    let mut guesses = vec![0; len];
    for i in 1..len {
        guesses[i] = i + 1
    }
    let answer = thread_rng().gen_range(1..len + 1);

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
        println!("Pick a number between 1 & {} :)", guesses.len());

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
    let guess_count = len - guesses.len();

    if guess_count > 1 {
        println!("You took {} guesses.", guess_count)
    } else {
        println!("First try. You legend")
    }

    Ok(())
}
