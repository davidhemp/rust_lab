fn dot_product(x: &[i32], y: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..x.len() {
        result += x[i] * y[i];
    }
    result
}
fn cross_product(x: &[i32], y: &[i32]) -> [i32; 3] {
    let mut z = [0; 3];
    // Cross product formula: z = x × y
    // Manual map for 3 element arrays
    z[0] = x[1] * y[2] - x[2] * y[1];
    z[1] = x[2] * y[0] - x[0] * y[2];
    z[2] = x[0] * y[1] - x[1] * y[0];
    z
}


fn main() {
    let x = [1, 2, 3];
    let y = [4, 5, 6];
    let z = [7, 8, 9];
    let result = dot_product(&x, cross_product(&y, &z));
    println!("The result of the triple product is: {}", result);

}