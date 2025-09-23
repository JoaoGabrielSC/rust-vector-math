mod vector;

use crate::vector::Vector;

fn main() {
    let a = Vector(vec![9.0, 2.0, 3.0]);
    let b = Vector(vec![1.0, 9.0, 3.0]);

    let c = a.dot_product(&b).unwrap();
    let d = a.norm().unwrap();

    let e = a.scalar_projection(&b).unwrap();

    println!("=== Vector Math ===");
    println!("Vector A {:?}", a);
    println!("Vector B {:?}", b);
    println!("Norm {:?}", d);
    println!("Dot product {}", c);
    println!("Scalar projection {}", e);
    println!("===================");
}
