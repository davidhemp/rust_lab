// Description: This program computes the cross product of two vectors in 3D space.

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
    let z = cross_product(&x, &y);

    for i in 0..3 {
        println!("z[{}] = {}", i, z[i]);
    }
}