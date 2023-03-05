use rand::{thread_rng, Rng};
use std::io;

fn main() -> io::Result<()> {
    let answer: u32 = thread_rng().gen_range(1..11);
    let mut running = true;

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

    println!("Correct!");

    Ok(())
}
