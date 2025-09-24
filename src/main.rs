use vector_math::matrix::Matrix;

mod vector;

fn main() {
    let a = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]).unwrap();

    println!("Matrix A:\n{}", a);

    let at = a.transpose();
    println!("A^T:\n{}", at);

    let c = a.mul(&at).unwrap();
    println!("A * A^T:\n{}", c);

    let (u, s, vt) = a.svd(1e-6, 1000).unwrap();
    println!("U:\n{}", u);
    println!("Singular values: {:?}", s);
    println!("V^T:\n{}", vt);
}
