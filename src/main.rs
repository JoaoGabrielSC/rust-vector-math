mod vector;

use crate::vector::Vector;

fn main() {
    let a = Vector(vec![1.0, 2.0, 3.0]);
    let b = Vector(vec![1.0, 2.0, 3.0]);

    let c = a.dot_product(&b).unwrap();

    println!("{}", c)
}
