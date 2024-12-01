use crate::matrix_err::MatrixErr;
use rand::thread_rng;
use rand::Rng;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut m_zeros = Matrix::zeros(rows, cols);

        m_zeros.randomize();

        m_zeros
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        let data: Vec<Vec<f64>> = vec![vec![0.0; cols as usize]; rows as usize];

        Matrix {
            rows,
            cols,
            data: data,
        }
    }

    pub fn add(m_a: &Matrix, m_b: &Matrix) -> Result<Matrix, MatrixErr> {
        Matrix::even_operations(m_a, m_b, |a,b| {a+b})
    }

    pub fn hadamard (m_a: &Matrix, m_b: &Matrix) -> Result<Matrix, MatrixErr> {
        Matrix::even_operations(m_a, m_b, |a,b| {a*b})
    }

    pub fn sub (m_a: &Matrix, m_b: &Matrix) -> Result<Matrix, MatrixErr> {
        Matrix::even_operations(m_a, m_b, |a,b| {a-b})
    }


    pub fn even_operations(m_a: &Matrix, m_b: &Matrix, func : fn(f64,f64)-> f64) -> Result<Matrix, MatrixErr> {
        if m_a.cols != m_b.cols || m_a.rows != m_b.rows {
            return Err(MatrixErr::AddDiferentSizeMatrixs);
        }

        let mut m_res = Matrix::zeros(m_a.rows, m_a.cols);

        for r in 0..m_res.rows as usize {
            for c in 0..m_res.cols as usize {
                m_res.data[r][c] = func(m_a.data[r][c], m_b.data[r][c])
            }
        }

        Ok(m_res)
    }

    pub fn mult(m_a: &Matrix, m_b: &Matrix) -> Result<Matrix, MatrixErr> {
        if m_a.cols != m_b.rows {
            return Err(MatrixErr::ImcompatibleMultiplicationMatrixs);
        }

        let mut m_res = Matrix::zeros(m_a.rows, m_b.cols);

        for a_r in 0..m_a.rows as usize {
            for b_c in 0..m_b.cols as usize {
                for ab in 0..m_b.rows as usize {
                    m_res.data[a_r][b_c] += m_a.data[a_r][ab] * m_b.data[ab][b_c]
                }
            }
        }

        Ok(m_res)
    }

    pub fn vector_to_matrix(vector: Vec<f64>) -> Matrix {
        let mut matrix_res = Matrix::zeros(vector.len(), 1);

        for row in 0..matrix_res.rows {
            matrix_res.data[row][0] = vector[row];
        }

        matrix_res
    }

    fn randomize(&mut self) {
        self.map(|_| thread_rng().gen());
    }

    pub fn scalar(&mut self, scalar: f64) {
        for row_i in self.data.iter_mut() {
            for col_i in row_i.iter_mut() {
                *col_i = *col_i * scalar;
            }
        }
    }

    pub fn map(&mut self, func: fn(f64) -> f64) {
        for row_i in self.data.iter_mut() {
            for col_i in row_i.iter_mut() {
                *col_i = func(*col_i);
            }
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut m_res = Matrix::zeros(self.cols, self.rows);

        for r in 0..self.rows {
            for c in 0..self.cols {
                m_res.data[c][r] = self.data[r][c]
            }
        }

        m_res
    }

    pub fn print(&self) {
        print!("\n");
        for row_v in &self.data {
            for col_v in row_v {
                print!("|{:.3} ", col_v);
            }
            print!("|\n");
        }
        print!("\n");
    }
}

