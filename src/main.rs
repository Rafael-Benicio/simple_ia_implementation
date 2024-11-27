mod matrix;

use crate::matrix::Matrix;

fn main() {
    let matrix_1 = Matrix::new(3, 3);
    let matrix_2 = Matrix::zeros(3, 3);

    matrix_1.print();
    matrix_2.print();

    let add_ab = Matrix::add(&matrix_1, &matrix_2).unwrap();

    add_ab.print();

    println!("Hello, world!");
}
