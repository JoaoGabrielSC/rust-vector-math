use core::fmt;
use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

#[allow(dead_code)]
pub enum VectorOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub struct Vector<T>(pub Vec<T>);

impl<T> Vector<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq,
{
    pub fn add(&self, other: &Self) -> Result<Self, &'static str> {
        operate_vectors(&self.0, &other.0, VectorOp::Add)
    }
    pub fn sub(&self, other: &Self) -> Result<Self, &'static str> {
        operate_vectors(&self.0, &other.0, VectorOp::Sub)
    }

    pub fn mul(&self, other: &Self) -> Result<Self, &'static str> {
        operate_vectors(&self.0, &other.0, VectorOp::Mul)
    }

    pub fn div(&self, other: &Self) -> Result<Self, &'static str> {
        operate_vectors(&self.0, &other.0, VectorOp::Div)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Vector<T>
where
    T: Float + fmt::Display,
{
    pub fn dot_product(&self, other: &Self) -> Result<T, &'static str> {
        if self.0.len() != other.0.len() {
            return Err("Vectors must have the same length");
        }
        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .fold(T::zero(), |acc, (&x, &y)| acc + x * y);

        Ok(result)
    }

    pub fn norm(&self) -> Result<T, &'static str> {
        if self.0.len() == 0 {
            return Err("Vector must have at least one element");
        }
        let result = self.0.iter().fold(T::zero(), |acc, &x| acc + x * x);

        Ok(result.sqrt())
    }

    pub fn unit_vect(&self) -> Result<Self, &'static str> {
        if self.0.len() == 0 {
            return Err("Vector must have at least one element");
        }

        let norm = self.norm()?;
        if norm == T::zero() {
            return Err("Cannot normalize zero vector");
        }
        let result = self.0.iter().map(|&x| x / norm).collect();

        Ok(Vector(result))
    }

    pub fn scalar_projection(&self, other: &Self) -> Result<T, &'static str> {
        if self.0.len() != other.0.len() {
            return Err("Vectors must have the same length");
        }

        let b_unit = other.unit_vect()?;
        let result = self.dot_product(&b_unit)?;

        Ok(result)
    }

    pub fn cosine_similarity(&self, other: &Self) -> Result<T, &'static str> {
        if self.0.len() != other.0.len() {
            return Err("Vectors must have the same length");
        }

        let dot = self.dot_product(other)?;
        let norm_a = self.norm()?;
        let norm_b = other.norm()?;

        if norm_a == T::zero() || norm_b == T::zero() {
            return Err("Cannot compute cosine similarity with zero vector");
        }

        Ok(dot / (norm_a * norm_b))
    }
}

impl<T> fmt::Display for Vector<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements = self
            .0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", elements)
    }
}

#[allow(dead_code)]
pub fn add_vectors(a: &Vec<f64>, b: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
    if a.len() != b.len() {
        return Err("Vectors must have the same length");
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect())
}

#[allow(dead_code)]
pub fn sub_vectors(a: &Vec<f64>, b: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
    if a.len() != b.len() {
        return Err("Vectors must have the same length");
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect())
}

#[allow(dead_code)]
pub fn mult_vectors(a: &Vec<f64>, b: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
    if a.len() != b.len() {
        return Err("Vectors must have the same length");
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect())
}

#[allow(dead_code)]
pub fn operate_vectors<T>(a: &[T], b: &[T], op: VectorOp) -> Result<Vector<T>, &'static str>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq,
{
    if a.len() != b.len() {
        return Err("Vectors must have the same length");
    }

    let result = a
        .iter()
        .zip(b.iter())
        .try_fold(Vec::new(), |mut acc, (&x, &y)| {
            let val = match op {
                VectorOp::Add => x + y,
                VectorOp::Sub => x - y,
                VectorOp::Mul => x * y,
                VectorOp::Div => {
                    if y == T::default() {
                        return Err("Division by zero");
                    }
                    x / y
                }
            };
            acc.push(val);
            Ok(acc)
        })?;
    Ok(Vector(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_sum_correctly() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let b: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(add_vectors(&a, &b), Ok(vec![2.0, 4.0, 6.0, 8.0]));
    }
    #[test]
    fn it_should_sub_correctly() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let b: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(sub_vectors(&a, &b), Ok(vec![0.0, 0.0, 0.0, 0.0]));
    }
    #[test]
    fn it_should_multiply_correctly() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let b: Vec<f64> = vec![2.0, 3.0, 4.0, 5.0];
        assert_eq!(mult_vectors(&a, &b), Ok(vec![2.0, 6.0, 12.0, 20.0]));
    }

    #[test]
    fn it_should_operate_add_correctly() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![1.0, 2.0, 3.0, 4.0];

        let result = operate_vectors(&a, &b, VectorOp::Add).unwrap();
        assert_eq!(result, Vector(vec![2.0, 4.0, 6.0, 8.0]));
    }

    #[test]
    fn it_should_operate_sub_correctly() {
        let a = vec![5.0, 6.0, 7.0, 8.0];
        let b = vec![1.0, 2.0, 3.0, 4.0];

        let result = operate_vectors(&a, &b, VectorOp::Sub).unwrap();
        assert_eq!(result, Vector(vec![4.0, 4.0, 4.0, 4.0]));
    }

    #[test]
    fn it_should_operate_mul_correctly() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![2.0, 3.0, 4.0, 5.0];

        let result = operate_vectors(&a, &b, VectorOp::Mul).unwrap();
        assert_eq!(result, Vector(vec![2.0, 6.0, 12.0, 20.0]));
    }

    #[test]
    fn it_should_operate_div_correctly() {
        let a = vec![2.0, 4.0, 6.0, 8.0];
        let b = vec![2.0, 2.0, 3.0, 4.0];
        let result = operate_vectors(&a, &b, VectorOp::Div).unwrap();
        assert_eq!(result, Vector(vec![1.0, 2.0, 2.0, 2.0]));
    }

    #[test]
    fn it_should_return_error_for_division_by_zero() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 0.0, 3.0];

        let result = operate_vectors(&a, &b, VectorOp::Div);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn it_should_return_dot_product_correctly() {
        let a = Vector(vec![1.0, 2.0, 3.0]);
        let b = Vector(vec![1.0, 2.0, 3.0]);
        let result = a.dot_product(&b).unwrap();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn it_should_return_error_for_different_lengths() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];

        let result = operate_vectors(&a, &b, VectorOp::Add);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Vectors must have the same length");
    }

    #[test]
    fn it_should_display_vector_correctly() {
        let v = Vector(vec![1, 2, 3, 4]);
        assert_eq!(format!("{}", v), "[1, 2, 3, 4]");
    }

    #[test]
    fn it_should_display_len_correctly() {
        let v = Vector(vec![1, 2, 3, 4]);
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn it_should_return_norm_correctly() {
        let v = Vector(vec![3.0, 4.0]);
        let result = v.norm().unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn it_should_return_unit_vector_correctly() {
        let v = Vector(vec![3.0, 4.0]);
        let result = v.unit_vect().unwrap();
        assert_eq!(result, Vector(vec![0.6, 0.8]));
    }

    #[test]
    fn it_should_return_scalar_projection_correctly() {
        let a = Vector(vec![3.0, 4.0]);
        let b = Vector(vec![1.0, 0.0]);
        let result = a.scalar_projection(&b).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn it_should_return_error_for_unit_vector_of_zero_vector() {
        let v = Vector(vec![0.0, 0.0]);
        let result = v.unit_vect();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot normalize zero vector");
    }
}
