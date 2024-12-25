use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    // println!("Hi, what is your name?:");
    // let mut name = String::new();
    
    // io::stdin().read_line(&mut name).expect("Failed to read line");
    
    // println!("Hello, {}", name.trim());
    // 
    // 
    let correct_num = rand::thread_rng().gen_range(1..=10); // Generate a random number between 1 and 10
        println!("Guess a number 1-10:");
        
        loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };
    
        // let message = if correct_num < guess {
        //     String::from("You guessed too high!")
        // } else if correct_num > guess {
        //     String::from("You guessed too low!")
        // } else {
        //     String::from("You guessed correctly!")
        // };
        let message = match guess.cmp(&correct_num){
            Ordering::Greater => "You guessed too high.",
            Ordering::Less => "You guessed too low",
            Ordering::Equal => {
                println!("You guessed correctly");
                break;
            }
        };
        println!("{message}")
        }
    }
