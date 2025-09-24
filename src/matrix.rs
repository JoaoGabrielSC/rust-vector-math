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
        for r in &data {
            if r.len() != cols {
                return Err("All rows must have the same number of columns");
            }
        }
        let rows = data.len();
        Ok(Matrix { data, rows, cols })
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
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

    pub fn mul_vec(&self, v: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
        if self.cols != v.len() {
            return Err("Incompatible shapes for matrix-vector multiplication");
        }
        let mut out = vec![0.0; self.rows];
        for i in 0..self.rows {
            let mut s = 0.0;
            for j in 0..self.cols {
                s += self.data[i][j] * v[j];
            }
            out[i] = s;
        }
        Ok(out)
    }

    pub fn col(&self, idx: usize) -> Vec<f64> {
        (0..self.rows).map(|r| self.data[r][idx]).collect()
    }

    pub fn set_col(&mut self, idx: usize, col: &Vec<f64>) -> Result<(), &'static str> {
        if col.len() != self.rows {
            return Err("Column length mismatch");
        }
        for r in 0..self.rows {
            self.data[r][idx] = col[r];
        }
        Ok(())
    }

    pub fn svd(
        &self,
        tol: f64,
        max_iter: usize,
    ) -> Result<(Matrix, Vec<f64>, Matrix), &'static str> {
        let a_t = self.transpose();
        let ata = a_t.mul(self)?; // n x n
        let n = ata.rows;
        let mut ata_work = ata.data.clone();
        let mut eigvals: Vec<f64> = Vec::new();
        let mut eigvecs: Vec<Vec<f64>> = Vec::new();

        for k in 0..n {
            let mut b_k = vec![1.0; n];
            let mut norm = b_k.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm == 0.0 {
                break;
            }
            for v in b_k.iter_mut() {
                *v /= norm;
            }
            let mut lambda = 0.0;
            for _ in 0..max_iter {
                let mut b_next = vec![0.0; n];
                for i in 0..n {
                    let mut s = 0.0;
                    for j in 0..n {
                        s += ata_work[i][j] * b_k[j];
                    }
                    b_next[i] = s;
                }
                let norm_bnext = b_next.iter().map(|x| x * x).sum::<f64>().sqrt();
                if norm_bnext == 0.0 {
                    break;
                }
                for v in b_next.iter_mut() {
                    *v /= norm_bnext;
                }
                let mut num = 0.0;
                for i in 0..n {
                    let mut s = 0.0;
                    for j in 0..n {
                        s += ata_work[i][j] * b_next[j];
                    }
                    num += b_next[i] * s;
                }
                let lambda_next = num;
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

        let mut singular_values: Vec<f64> = eigvals
            .iter()
            .map(|&x| if x > 0.0 { x.sqrt() } else { 0.0 })
            .collect();

        let r = singular_values.len();
        if r == 0 {
            // all zeros
            let u = Matrix::zeros(self.rows, self.rows);
            let v_t = Matrix::zeros(0, 0);
            return Ok((u, vec![], v_t));
        }
        let mut v_mat = Matrix::zeros(n, r);
        for (j, vecj) in eigvecs.iter().enumerate() {
            v_mat.set_col(j, vecj)?;
        }

        let av = self.mul(&v_mat)?;
        let mut u_mat = Matrix::zeros(self.rows, r);
        for j in 0..r {
            let sigma = singular_values[j];
            if sigma.abs() < 1e-12 {
                u_mat.set_col(j, &vec![0.0; self.rows])?;
            } else {
                let col_aj = av.col(j);
                let scaled: Vec<f64> = col_aj.iter().map(|x| x / sigma).collect();
                u_mat.set_col(j, &scaled)?;
            }
        }

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
