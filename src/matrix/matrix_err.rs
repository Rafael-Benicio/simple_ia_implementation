use core::fmt::Formatter;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum MatrixErr {
    AddDiferentSizeMatrixs,
    ImcompatibleMultiplicationMatrixs,
}

impl Error for MatrixErr {}

impl Display for MatrixErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MatrixErr::AddDiferentSizeMatrixs => {
                write!(f, "Error in Add Matrixs, COL_a != Col_b or Row_a != Row_b",)
            }
            MatrixErr::ImcompatibleMultiplicationMatrixs => {
                write!(f, "Error in Multiply Matrixs, Col_a != Row_b",)
            }
        }
    }
}
