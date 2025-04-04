// Description: This program computes the cross product of two vectors in 3D space.

fn main() {
    let x = [1, 2, 3, 4, 5];
    let y = [6, 7, 8, 9, 10];
    let mut z = [[0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            println!("z[{}]: {}", i, z[i][j]);
        }
    }
}