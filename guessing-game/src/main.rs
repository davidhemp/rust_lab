use rand::Rng;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let guess = rand::rng().random_range(1..=100);

    println!("You guessed: {}", guess);
    
}