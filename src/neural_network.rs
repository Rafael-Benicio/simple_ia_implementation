use crate::math_functions::MathFunctions;
use crate::Matrix;

#[derive(Debug)]
pub struct NeuralNetwork {
    i_nodes: usize,
    h_nodes: usize,
    o_nodes: usize,
    bias_ih: Matrix,
    bias_ho: Matrix,
    weight_ih: Matrix,
    weight_oh: Matrix,
    learning_rate: f64,
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
            learning_rate: 1.0,
        }
    }

    pub fn train(&mut self, input: Vec<f64>, target: Vec<f64>) {
        let input = Matrix::vector_to_matrix(&input);
        let mut hidden = Matrix::mult(&self.weight_ih, &input);
        hidden = Matrix::add(&hidden, &self.bias_ih);

        hidden.map(MathFunctions::sigmoid);

        let mut output = Matrix::mult(&self.weight_oh, &hidden);
        output = Matrix::add(&output, &self.bias_ho);

        output.map(MathFunctions::sigmoid);

        // back propagation

        let expected = Matrix::vector_to_matrix(&target);

        let o_error = Matrix::sub(&expected, &output);
        let mut d_output = output.clone();

        d_output.map(MathFunctions::d_sigmoid);

        let hidden_t = Matrix::transpose(&hidden);
        let mut gradient = Matrix::hadamard(&o_error, &d_output);

        gradient.scalar(self.learning_rate);

        let w_ho_deltas = Matrix::mult(&gradient, &hidden_t);

        self.weight_oh = Matrix::add(&self.weight_oh, &w_ho_deltas);

        // =======================

        let w_oh_t = self.weight_oh.transpose();

        let hidden_error = Matrix::mult(&w_oh_t, &o_error);
        let mut d_hidden = hidden.clone();

        d_hidden.map(MathFunctions::d_sigmoid);

        let input_t = input.transpose();

        let mut gradient_h = Matrix::hadamard(&hidden_error, &d_hidden);
        gradient_h.scalar(self.learning_rate);

        let w_ih_deltas = Matrix::mult(&gradient_h, &input_t);

        self.weight_ih = Matrix::add(&self.weight_ih, &w_ih_deltas);
    }

    pub fn predict(&self, input: Vec<f64>) -> Vec<f64> {
        let input = Matrix::vector_to_matrix(&input);
        let mut hidden = Matrix::mult(&self.weight_ih, &input);
        hidden = Matrix::add(&hidden, &self.bias_ih);

        hidden.map(MathFunctions::sigmoid);

        let mut output = Matrix::mult(&self.weight_oh, &hidden);
        output = Matrix::add(&output, &self.bias_ho);

        output.map(MathFunctions::sigmoid);

        Matrix::matrix_to_vector(&output)
    }
}
