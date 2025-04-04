use rand::Rng;

fn main() {
    let x = [1, 2, 3, 4, 5];
    let y = [6, 7, 8, 9, 10];
    let mut z = [0, 0, 0, 0, 0];
    for i in 0..5 {
        z[i] = x[i] * y[i];
    }
    println!("The dot product is: {}", z.iter().sum::<i32>());
}