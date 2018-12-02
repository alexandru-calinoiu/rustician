extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value: value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your gues.");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number please try again.");
                continue;
            }
        };

        let guess = Guess::new(guess);

        println!("Your guess is {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("Right on!");
                break;
            }
        }
    }
}
