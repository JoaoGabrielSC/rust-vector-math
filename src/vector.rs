use core::fmt;
use std::ops::{Add, Div, Mul, Sub};

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

    pub fn dot_product(&self, other: &Self) -> Result<T, &'static str>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Default,
    {
        dot_prod(&self.0, &other.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
pub enum VectorOp {
    Add,
    Sub,
    Mul,
    Div,
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

pub fn dot_prod<T>(a: &[T], b: &[T]) -> Result<T, &'static str>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
{
    if a.len() != b.len() {
        return Err("Vectors must have the same length");
    }

    let result = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| *x * *y)
        .fold(T::default(), |acc, val| acc + val);

    Ok(result)
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
}
