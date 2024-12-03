use std::default::Default;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, Sub};

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Debug
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + Default
        + Display,
    Standard: Distribution<T>,
{
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        let mut m_zeros = Matrix::zeros(rows, cols);

        m_zeros.randomize();

        m_zeros
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix<T> {
        if rows == 0 || cols == 0 {
            panic!("Row or column number is 0");
        }

        let data: Vec<Vec<T>> = vec![vec![Default::default(); cols as usize]; rows as usize];

        Matrix {
            rows,
            cols,
            data: data,
        }
    }

    pub fn add(m_a: &Matrix<T>, m_b: &Matrix<T>) -> Matrix<T> {
        Matrix::even_operations(m_a, m_b, |a, b| a + b)
    }

    pub fn sub(m_a: &Matrix<T>, m_b: &Matrix<T>) -> Matrix<T> {
        Matrix::even_operations(m_a, m_b, |a, b| a - b)
    }

    pub fn hadamard(m_a: &Matrix<T>, m_b: &Matrix<T>) -> Matrix<T> {
        Matrix::even_operations(m_a, m_b, |a, b| a * b)
    }

    pub fn even_operations(m_a: &Matrix<T>, m_b: &Matrix<T>, func: fn(T, T) -> T) -> Matrix<T> {
        if m_a.cols != m_b.cols || m_a.rows != m_b.rows {
            panic!("Error in same size Matrix operation")
        }

        let mut m_res = Matrix::zeros(m_a.rows, m_a.cols);

        for r in 0..m_res.rows as usize {
            for c in 0..m_res.cols as usize {
                m_res.data[r][c] = func(m_a.data[r][c], m_b.data[r][c])
            }
        }

        m_res
    }

    pub fn mult(m_a: &Matrix<T>, m_b: &Matrix<T>) -> Matrix<T> {
        if m_a.cols != m_b.rows {
            panic!("Multiplication with matrix of incompatibles sizes")
        }

        let mut m_res = Matrix::zeros(m_a.rows, m_b.cols);

        for a_r in 0..m_a.rows as usize {
            for b_c in 0..m_b.cols as usize {
                for ab in 0..m_b.rows as usize {
                    m_res.data[a_r][b_c] += m_a.data[a_r][ab] * m_b.data[ab][b_c]
                }
            }
        }

        m_res
    }

    pub fn vector_to_matrix(vector: &Vec<T>) -> Matrix<T> {
        let mut matrix_res = Matrix::zeros(vector.len(), 1);

        for row in 0..matrix_res.rows {
            matrix_res.data[row][0] = vector[row];
        }

        matrix_res
    }

    pub fn matrix_to_vector(matrix: &Matrix<T>) -> Vec<T> {
        let mut vec = vec![Default::default(); matrix.rows * matrix.cols];
        let mut index = 0;

        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                vec[index] = matrix.data[row][col];
                index += 1;
            }
        }

        vec
    }

    fn randomize(&mut self) {
        self.map(|_| thread_rng().gen());
    }

    pub fn scalar(&mut self, scalar: T) {
        for row_i in self.data.iter_mut() {
            for col_i in row_i.iter_mut() {
                *col_i = *col_i * scalar;
            }
        }
    }

    pub fn map(&mut self, func: fn(T) -> T) {
        for row_i in self.data.iter_mut() {
            for col_i in row_i.iter_mut() {
                *col_i = func(*col_i);
            }
        }
    }

    pub fn transpose(&self) -> Matrix<T> {
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
                print!("|{} ", col_v);
            }
            print!("|\n");
        }
        print!("\n");
    }
}
