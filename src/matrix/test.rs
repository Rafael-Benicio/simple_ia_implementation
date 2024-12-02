#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn zeros() {
        let m_1 = Matrix::zeros(1, 1);
        let m_2 = Matrix::zeros(2, 1);
        let m_3 = Matrix::zeros(3, 1);
        let m_4 = Matrix::zeros(3, 5);

        let full_zeros = |m: Matrix| -> bool {
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

    //     #[test]
    //     #[should_panic]
    //     fn negative_height() {
    //     let _rect = Rectangle::new(10, -10);
    //     }
}
