#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    #[should_panic]
    fn zeros_panic_1() {
        Matrix::<f64>::zeros(1, 0);
    }

    #[test]
    #[should_panic]
    fn zeros_panic_2() {
        Matrix::<f64>::zeros(0, 1);
    }

    #[test]
    #[should_panic]
    fn zeros_panic_3() {
        Matrix::<f64>::zeros(0, 0);
    }

    #[test]
    fn zeros() {
        let m_1 = Matrix::zeros(1, 1);
        let m_2 = Matrix::zeros(2, 1);
        let m_3 = Matrix::zeros(3, 1);
        let m_4 = Matrix::zeros(3, 5);

        let full_zeros = |m: Matrix<f64>| -> bool {
            for m_i in m.data {
                for m_j in m_i {
                    if m_j != 0.0 {
                        return false;
                    }
                }
            }

            true
        };
        // checa se os resultados são iguais
        assert_eq!(m_1.rows, 1);
        assert_eq!(m_1.cols, 1);
        assert_eq!(m_2.rows, 2);
        assert_eq!(m_2.cols, 1);
        assert_eq!(m_3.rows, 3);
        assert_eq!(m_3.cols, 1);
        assert_eq!(m_4.rows, 3);
        assert_eq!(m_4.cols, 5);

        assert!(full_zeros(m_1));
        assert!(full_zeros(m_2));
        assert!(full_zeros(m_3));
        assert!(full_zeros(m_4));
    }

    #[test]
    fn add() {
        let mut m_1 = Matrix::zeros(2, 2);
        let mut m_2 = Matrix::zeros(2, 2);
        let mut m_3 = Matrix::zeros(2, 2);
        m_1.map(|_| 1.0);
        m_2.map(|_| 2.0);
        m_3.map(|_| 3.0);

        let m_res = Matrix::even_operations(&Matrix::add(&m_1, &m_2), &m_3, |a, b| {
            if a == b {
                1.0
            } else {
                0.0
            }
        });

        for m in m_res.data {
            assert_eq!(m.contains(&0.0), false)
        }
    }

    #[test]
    fn sub() {
        let mut m_1 = Matrix::zeros(2, 2);
        let mut m_2 = Matrix::zeros(2, 2);
        let mut m_3 = Matrix::zeros(2, 2);
        m_1.map(|_| 5.0);
        m_2.map(|_| 2.0);
        m_3.map(|_| 3.0);

        let m_res = Matrix::even_operations(&Matrix::sub(&m_1, &m_2), &m_3, |a, b| {
            if a == b {
                1.0
            } else {
                0.0
            }
        });

        for m in m_res.data {
            assert_eq!(m.contains(&0.0), false)
        }
    }

    #[test]
    fn hadamard() {
        let mut m_1 = Matrix::zeros(2, 2);
        let mut m_2 = Matrix::zeros(2, 2);
        let mut m_3 = Matrix::zeros(2, 2);
        m_1.map(|_| 1.0);
        m_2.map(|_| 2.0);
        m_3.map(|_| 2.0);

        let m_res = Matrix::even_operations(&Matrix::hadamard(&m_1, &m_2), &m_3, |a, b| {
            if a == b {
                1.0
            } else {
                0.0
            }
        });

        for m in m_res.data {
            assert_eq!(m.contains(&0.0), false)
        }
    }

    #[test]
    fn mult() {
        let m_1: Matrix<f64> = Matrix::zeros(2, 3);
        let m_2 = Matrix::zeros(3, 2);

        let m_res = Matrix::mult(&m_1, &m_2);

        assert_eq!(m_res.rows, m_1.rows);
        assert_eq!(m_res.cols, m_2.cols);
    }

    #[test]
    #[should_panic]
    fn mult_panic() {
        let m_1: Matrix<f64> = Matrix::zeros(2, 2);
        let m_2 = Matrix::zeros(3, 2);

        let m_res = Matrix::mult(&m_1, &m_2);

        assert_eq!(m_res.rows, m_1.rows);
        assert_eq!(m_res.cols, m_2.cols);
    }

    #[test]
    #[should_panic]
    fn hadamard_panic() {
        let mut m_1 = Matrix::zeros(2, 2);
        let mut m_2 = Matrix::zeros(2, 1);
        let mut m_3 = Matrix::zeros(2, 2);
        m_1.map(|_| 1);
        m_2.map(|_| 2);
        m_3.map(|_| 2);

        let m_res = Matrix::even_operations(&Matrix::hadamard(&m_1, &m_2), &m_3, |a, b| {
            if a == b {
                1
            } else {
                0
            }
        });

        for m in m_res.data {
            assert_eq!(m.contains(&0), false)
        }
    }

    #[test]
    fn vector_to_matrix() {
        let test_v = vec![1, 2, 4];

        let m_test_v: Matrix<u8> = Matrix::vector_to_matrix(&test_v);

        assert_eq!(m_test_v.cols, 1);
        assert_eq!(m_test_v.rows, test_v.len());
    }

    #[test]
    fn matrix_to_vector() {
        let test_vec_1: Vec<f64> = Matrix::matrix_to_vector(&Matrix::zeros(4, 4));
        let test_vec_2: Vec<f64> = Matrix::matrix_to_vector(&Matrix::zeros(4, 2));
        let test_vec_3: Vec<f64> = Matrix::matrix_to_vector(&Matrix::zeros(2, 6));

        assert_eq!(test_vec_1.len(), 16);
        assert_eq!(test_vec_2.len(), 8);
        assert_eq!(test_vec_3.len(), 12);
    }

    #[test]
    fn map() {
        let mut m_1: Matrix<f64> = Matrix::zeros(2, 5);

        m_1.map(|_| 2.0);

        for i in m_1.data {
            for j in i {
                assert_eq!(j, 2.0);
            }
        }
    }

    #[test]
    fn transpose() {
        let m_1: Matrix<i8> = Matrix::zeros(3, 2);

        let m_1 = m_1.transpose();

        assert_eq!(m_1.rows, 2);
        assert_eq!(m_1.cols, 3);
    }

    #[test]
    fn scalar() {
        let mut m_1: Matrix<f64> = Matrix::zeros(3, 4);

        m_1.map(|_| 1.0);

        m_1.scalar(4.0);

        for i in m_1.data.iter() {
            for j in i {
                assert_eq!(*j, 4.0);
            }
        }

        m_1.map(|_| 1.0);
        m_1.scalar(1.5);

        for i in &m_1.data {
            for j in i {
                assert_eq!(*j, 1.5);
            }
        }
    }
}
