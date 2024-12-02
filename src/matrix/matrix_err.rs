use core::fmt::Formatter;
use std::error::Error;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MatrixErr {
    DiferentSizeMatrixs,
    ImcompatibleMultiplicationMatrixs,
}

impl Error for MatrixErr {}

impl Display for MatrixErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MatrixErr::DiferentSizeMatrixs => {
                write!(
                    f,
                    "Error in Matrixs operation, COL_a != Col_b or Row_a != Row_b",
                )
            }
            MatrixErr::ImcompatibleMultiplicationMatrixs => {
                write!(f, "Error in Multiply Matrixs, Col_a != Row_b",)
            }
        }
    }
}
