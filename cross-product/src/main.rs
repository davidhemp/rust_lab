// Description: This program computes the cross product of two vectors in 3D space.

fn main() {
    let x = [1, 2, 3];
    let y = [4, 5, 6];
    let mut z = [0; 3];

    // Cross product formula: z = x × y
    // Manaul map for 3 element arrays
    z[0] = x[1] * y[2] - x[2] * y[1];
    z[1] = x[0] * y[2] - x[2] * y[0];
    z[2] = x[0] * y[1] - x[1] * y[0];
    for i in 0..3 {
        println!("z[{}] = {}", i, z[i]);
    }
}