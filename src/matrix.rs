use std::fmt;

/// Simple Matrix type for numerical ops (f64)
#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self, &'static str> {
        if data.is_empty() || data[0].is_empty() {
            return Err("Matrix cannot be empty");
        }
        let cols = data[0].len();
        if !data.iter().all(|r| r.len() == cols) {
            return Err("All rows must have the same number of columns");
        }
        Ok(Matrix {
            rows: data.len(),
            cols,
            data,
        })
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut t = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                t[j][i] = self.data[i][j];
            }
        }
        Matrix {
            data: t,
            rows: self.cols,
            cols: self.rows,
        }
    }

    pub fn mul(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != other.rows {
            return Err("Incompatible shapes for multiplication");
        }
        let mut out = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a = self.data[i][k];
                for j in 0..other.cols {
                    out[i][j] += a * other.data[k][j];
                }
            }
        }
        Ok(Matrix {
            data: out,
            rows: self.rows,
            cols: other.cols,
        })
    }

    pub fn mul_vec(&self, v: &[f64]) -> Result<Vec<f64>, &'static str> {
        if self.cols != v.len() {
            return Err("Incompatible shapes for matrix-vector multiplication");
        }
        Ok(self
            .data
            .iter()
            .map(|row| row.iter().zip(v).map(|(a, b)| a * b).sum())
            .collect())
    }

    pub fn col(&self, idx: usize) -> Vec<f64> {
        (0..self.rows).map(|r| self.data[r][idx]).collect()
    }

    pub fn set_col(&mut self, idx: usize, col: &[f64]) -> Result<(), &'static str> {
        if col.len() != self.rows {
            return Err("Column length mismatch");
        }
        for (r, val) in col.iter().enumerate() {
            self.data[r][idx] = *val;
        }
        Ok(())
    }

    fn normalize(v: &mut Vec<f64>) -> f64 {
        let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for val in v.iter_mut() {
                *val /= norm;
            }
        }
        norm
    }

    fn mat_vec_mul(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v).map(|(a, b)| a * b).sum())
            .collect()
    }

    fn rayleigh_quotient(m: &[Vec<f64>], v: &[f64]) -> f64 {
        let mv = Self::mat_vec_mul(m, v);
        v.iter().zip(mv.iter()).map(|(vi, mvi)| vi * mvi).sum()
    }

    pub fn svd(
        &self,
        tol: f64,
        max_iter: usize,
    ) -> Result<(Matrix, Vec<f64>, Matrix), &'static str> {
        let ata = self.transpose().mul(self)?;
        let n = ata.rows;
        let mut ata_work = ata.data.clone();

        let mut eigvals = Vec::new();
        let mut eigvecs = Vec::new();

        for _ in 0..n {
            let mut b_k = vec![1.0; n];
            if Self::normalize(&mut b_k) == 0.0 {
                break;
            }

            let mut lambda = 0.0;
            for _ in 0..max_iter {
                let mut b_next = Self::mat_vec_mul(&ata_work, &b_k);
                if Self::normalize(&mut b_next) == 0.0 {
                    break;
                }
                let lambda_next = Self::rayleigh_quotient(&ata_work, &b_next);
                if (lambda_next - lambda).abs() < tol {
                    lambda = lambda_next;
                    b_k = b_next;
                    break;
                }
                lambda = lambda_next;
                b_k = b_next;
            }

            if lambda.abs() < 1e-12 {
                break;
            }

            eigvals.push(lambda);
            eigvecs.push(b_k.clone());

            for i in 0..n {
                for j in 0..n {
                    ata_work[i][j] -= lambda * b_k[i] * b_k[j];
                }
            }
        }

        let singular_values: Vec<f64> = eigvals
            .iter()
            .map(|&x| if x > 0.0 { x.sqrt() } else { 0.0 })
            .collect();
        let r = singular_values.len();
        if r == 0 {
            return Ok((
                Matrix::zeros(self.rows, self.rows),
                vec![],
                Matrix::zeros(0, 0),
            ));
        }

        let mut v_mat = Matrix::zeros(n, r);
        for (j, vecj) in eigvecs.iter().enumerate() {
            v_mat.set_col(j, vecj)?;
        }

        let av = self.mul(&v_mat)?;
        let mut u_mat = Matrix::zeros(self.rows, r);
        for (j, sigma) in singular_values.iter().enumerate() {
            let col_aj = av.col(j);
            let col = if sigma.abs() < 1e-12 {
                vec![0.0; self.rows]
            } else {
                col_aj.iter().map(|x| x / sigma).collect()
            };
            u_mat.set_col(j, &col)?;
        }

        // Build V^T
        let mut v_t = Matrix::zeros(r, n);
        for j in 0..r {
            let vj = v_mat.col(j);
            for i in 0..n {
                v_t.data[j][i] = vj[i];
            }
        }

        Ok((u_mat, singular_values, v_t))
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            let row_str = row
                .iter()
                .map(|x| format!("{:8.4}", x))
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(f, "[{}]", row_str)?;
        }
        Ok(())
    }
}
