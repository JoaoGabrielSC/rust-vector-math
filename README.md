# Vector Math - NumPy from Scratch in Rust

A "from scratch" implementation of vector mathematical operations in Rust, inspired by NumPy. This project provides a basic linear algebra library with generic types and fundamental vector operations.

## Features

### Basic Operations

- **Vector addition** (`+`)
- **Vector subtraction** (`-`)
- **Element-wise multiplication** (`*`)
- **Element-wise division** (`/`)

### Advanced Operations

- **Dot product**
- **Euclidean norm** (vector magnitude)
- **Unit vector** (normalization)
- **Scalar projection** of one vector onto another
- **Cosine similarity** between vectors

### Technical Features

- **Generic types** - Works with any numeric type
- **Error handling** - Dimension compatibility checks
- **Appropriate trait bounds** for mathematical operations
- **Input validation** - Prevents division by zero and incompatible vectors

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
vector_math = "0.1.0"
num-traits = "0.2"
```

## Basic Usage

```rust
use vector_math::vector::Vector;

fn main() {
    // Creating vectors
    let a = Vector(vec![1.0, 2.0, 3.0]);
    let b = Vector(vec![4.0, 5.0, 6.0]);
    
    // Basic operations
    let sum = a.add(&b).unwrap();           // [5.0, 7.0, 9.0]
    let subtraction = a.sub(&b).unwrap();   // [-3.0, -3.0, -3.0]
    let multiplication = a.mul(&b).unwrap(); // [4.0, 10.0, 18.0]
    let division = a.div(&b).unwrap();      // [0.25, 0.4, 0.5]
    
    // Advanced operations
    let dot_product = a.dot_product(&b).unwrap();  // 32.0
    let norm_a = a.norm().unwrap();                 // 3.742
    let unit_vector = a.unit_vect().unwrap();       // [0.267, 0.535, 0.802]
    let projection = a.scalar_projection(&b).unwrap(); // 2.201
    let similarity = a.cosine_similarity(&b).unwrap(); // 0.975
    
    println!("Dot product: {:.3}", dot_product);
    println!("Norm of vector A: {:.3}", norm_a);
    println!("Cosine similarity: {:.3}", similarity);
}
```

## Example Output

When running `cargo run`, you'll see:

```text
Vector A                       Vector([9.0, 2.0, 3.0])
Vector B                       Vector([1.0, 9.0, 3.0])
Norm of A                      9.695
Norm of B                      9.644
Dot product (A · B)            36.000
Scalar projection of A on B    3.732
Cosine similarity (A, B)       0.385

Vector F                       Vector([1.0, 2.0, 3.0])
Vector G                       Vector([4.0, 5.0, 6.0])
F + G                          Vector([5.0, 7.0, 9.0])
F - G                          Vector([-3.0, -3.0, -3.0])
F · G                          32.000
Norm of F                      3.742
Norm of G                      8.775
F * G                          Vector([4.0, 10.0, 18.0])
F / G                          Vector([0.25, 0.4, 0.5])
Length of F                    3
Length of G                    3
Cosine similarity (F, G)       0.975
```

## Running Tests

```bash
cargo test
```

The tests cover:

- Basic operations (addition, subtraction, multiplication, division)
- Dot product and norm
- Unit vectors and projections
- Error handling (division by zero, vectors of different sizes)
- Formatting and display

## Architecture

### Main Structure

```rust
pub struct Vector<T>(pub Vec<T>);
```

### Implemented Traits

- `Debug` - For debugging
- `PartialEq` - For comparisons
- `Display` - For custom formatting

### Type Constraints

The generic type `T` must implement:

- `Copy + Add + Sub + Mul + Div + Default + PartialEq` for basic operations
- `Float + fmt::Display` for advanced mathematical operations

## Future Features

- Multidimensional matrices
- Cross product
- Decompositions (SVD, LU, QR)
- Scalar operations
- SIMD for performance optimization
- BLAS/LAPACK integration
- Broadcasting for operations between vectors of different sizes

## Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## Educational Purpose

This project was created for educational purposes to:

- Understand the implementation of linear algebra operations
- Practice generic programming in Rust
- Explore trait bounds and type constraints
- Implement robust error handling

## Learning Resources

- [Rust Book - Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [NumPy Documentation](https://numpy.org/doc/)
- [Linear Algebra Concepts](https://betterexplained.com/articles/linear-algebra-guide/)

---

Made with Rust
