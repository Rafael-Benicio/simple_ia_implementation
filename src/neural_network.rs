use crate::math_functions::MathFunctions;
use crate::matrix::Matrix;

#[derive(Debug)]
pub struct NeuralNetwork {
    i_nodes: usize,
    h_nodes: usize,
    o_nodes: usize,
    bias_ih: Matrix,
    bias_ho: Matrix,
    weight_ih: Matrix,
    weight_oh: Matrix,
}

impl NeuralNetwork {
    pub fn new(i_size: usize, h_size: usize, o_size: usize) -> Self {
        NeuralNetwork {
            i_nodes: i_size,
            h_nodes: h_size,
            o_nodes: o_size,
            bias_ih: Matrix::new(h_size, 1),
            bias_ho: Matrix::new(o_size, 1),
            weight_ih: Matrix::new(h_size, i_size),
            weight_oh: Matrix::new(o_size, h_size),
        }
    }

    pub fn feedforward(&self, input: Vec<f64>) -> Matrix {
        let input = Matrix::vector_to_matrix(input);
        let mut hidden =
            Matrix::mult(&self.weight_ih, &input).expect("Erro in weight * input process");
        hidden = Matrix::add(&hidden, &self.bias_ih).expect("Erro in hidden + bias process");

        hidden.map(MathFunctions::sigmoid);

        let mut output =
            Matrix::mult(&self.weight_oh, &hidden).expect("Erro in hidden * weight process");
        output = Matrix::add(&output, &self.bias_ho).expect("Erro in output + bias process");

        output.map(MathFunctions::sigmoid);

        output
    }
}
