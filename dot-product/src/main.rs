use rand::Rng;

fn dot_product(x: &[i32], y: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..x.len() {
        result += x[i] * y[i];
    }
    result
}

fn main() {
    let x = [1, 2, 3, 4, 5];
    let y = [6, 7, 8, 9, 10];

    println!("The dot product is: {}", dot_product(&x, &y));
}