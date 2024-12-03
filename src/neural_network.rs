use crate::math_functions::MathFunctions;
use crate::Matrix;

#[derive(Debug)]
pub struct NeuralNetwork {
    i_nodes: usize,
    h_nodes: usize,
    o_nodes: usize,
    bias: Vec<Matrix<f64>>,
    weights : Vec<Matrix<f64>>,
    learning_rate: f64,
}

impl NeuralNetwork {
    pub fn new(i_size: usize, h_size: usize, o_size: usize) -> Self {
        NeuralNetwork {
            i_nodes: i_size,
            h_nodes: h_size,
            o_nodes: o_size,
            bias: vec![Matrix::new(h_size, 1),Matrix::new(o_size, 1)],
            weights: vec![Matrix::new(h_size, i_size),Matrix::new(o_size, h_size)],
            learning_rate: 1.0,
        }
    }

    pub fn train(&mut self, input: Vec<f64>, target: Vec<f64>) {
        let input = Matrix::vector_to_matrix(&input);
        let expected = Matrix::vector_to_matrix(&target);

        // Feedforwad

        let mut hidden = self.node_layer(&input,0);
        let mut output = self.node_layer(&hidden,1);

        // Backpropagation

        let error_b = Matrix::sub(&expected, &output);
        let hidden_t = Matrix::transpose(&hidden);

        output.map(MathFunctions::d_sigmoid);

        let mut gradient_b = Matrix::hadamard(&error_b, &output);
        gradient_b.scalar(self.learning_rate);

        let weight_deltas = Matrix::mult(&gradient_b, &hidden_t);

        self.weights[1] = Matrix::add(&self.weights[1], &weight_deltas);

        // =======================

        let weight_t = self.weights[1].transpose();

        let error_a = Matrix::mult(&weight_t, &error_b);
        let input_t = input.transpose();

        hidden.map(MathFunctions::d_sigmoid);

        let mut gradient_a = Matrix::hadamard(&error_a, &hidden);
        gradient_a.scalar(self.learning_rate);

        let weight_deltas = Matrix::mult(&gradient_a, &input_t);

        self.weights[0] = Matrix::add(&self.weights[0], &weight_deltas);
    }

    fn node_layer(&self, layer: &Matrix<f64>,layer_index : usize) -> Matrix<f64> {
        let mut output = Matrix::mult(&self.weights[layer_index], &layer);
        output = Matrix::add(&output, &self.bias[layer_index]);
        output.map(MathFunctions::sigmoid);
        output
    }

    pub fn predict(&self, input: Vec<f64>) -> Vec<f64> {
        let input = Matrix::vector_to_matrix(&input);

        let hidden = self.node_layer(&input,0);
        let output = self.node_layer(&hidden,1);

        Matrix::matrix_to_vector(&output)
    }
}
