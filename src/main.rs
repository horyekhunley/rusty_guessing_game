use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut how_many = String::new();

    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Failed to read line");

    let num_guesses: u8 = how_many.trim().parse().expect("Failed to parse");
    let mut correct_nums = Vec::new();

    for _ in 0..num_guesses {
        let num = rand::thread_rng().gen_range(1..=10);
        correct_nums.push(num);
    }

    let mut guesses_made = 0;

    while guesses_made < num_guesses {
        println!("Guess a number between 1 and 10:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match guess.cmp(&correct_nums[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low."),
            Ordering::Equal => {
                println!("You guessed correctly!");
                guesses_made += 1;

                if guesses_made < num_guesses {
                    println!(
                        "You have {} guesses left.",
                        num_guesses - guesses_made
                    );
                }
            }
        };
    }

    println!("Thanks for playing! The correct numbers were: {:?}", correct_nums);
}
