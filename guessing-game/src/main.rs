use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=10);
    let mut lower_bound = 1;
    let mut upper_bound = 10;

    loop {
        println!("Please input your guess.");
        let guess = rand::rng().random_range(lower_bound..=upper_bound);
    
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!")
                lower_bound = guess + 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                upper_bound = guess - 1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}