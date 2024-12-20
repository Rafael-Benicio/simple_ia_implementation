use core::f64::consts::E;

#[derive(Debug)]
pub struct MathFunctions {}

impl MathFunctions {
    pub fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + E.powf(-x))
    }

    pub fn d_sigmoid(x: f64) -> f64 {
        x * (1.0 - x)
    }
}
