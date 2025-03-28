use rand::Rng;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess.");

    let guess = rand::rng().random_range(1..=100);

    println!("You guessed: {}", guess);
    
}