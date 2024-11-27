use core::fmt::Formatter;
use std::error::Error;
use std::fmt::Display;

use rand::thread_rng;
use rand::Rng;

#[derive(Debug)]
pub enum MatrixErr{
    AddDiferentSizeMatrixs,
}

impl Error for MatrixErr{}

impl Display for MatrixErr{
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match *self {
            MatrixErr::AddDiferentSizeMatrixs => Ok(()),
        }
    }
}

pub struct Matrix {
    rows: i32,
    cols: i32,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: i32, cols: i32) -> Matrix {
        let data: Vec<Vec<f64>> = vec![vec![0.0; cols as usize]; rows as usize];
        let mut new_matrix = Matrix {
            rows,
            cols,
            data: data,
        };

        new_matrix.randomize();
        new_matrix
    }

    pub fn zeros(rows: i32, cols: i32) -> Matrix {
        let data: Vec<Vec<f64>> = vec![vec![0.0; cols as usize]; rows as usize];
        Matrix {
            rows,
            cols,
            data: data,
        }
    }

    fn randomize(&mut self) {
        for row_i in self.data.iter_mut() {
            for col_i in row_i.iter_mut() {
                *col_i = thread_rng().gen();
            }
        }
    }

    pub fn print(&self) {
        print!("\n");
        for row_v in &self.data {
            for col_v in row_v {
                print!("{} ", col_v);
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn add(matrix_a: &Matrix, matrix_b: &Matrix) -> Result<Matrix, MatrixErr> {
        if matrix_a.cols != matrix_b.cols || matrix_a.rows != matrix_b.rows {
            return Err(MatrixErr::AddDiferentSizeMatrixs);
        }

        let mut matrix_res = Matrix::zeros(matrix_a.rows, matrix_a.cols);

        for r in 0..matrix_res.rows as usize {
            for c in 0..matrix_res.cols as usize {
                matrix_res.data[r][c] = matrix_a.data[r][c] + matrix_b.data[r][c]
            }
        }

        Ok(matrix_res)
    }
}
