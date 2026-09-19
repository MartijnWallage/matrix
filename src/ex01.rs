use crate::Vector;
use crate::Matrix;

pub fn linear_combination(u: &[Vector<f64>], coefs: &[f64]) -> Vector<f64> {
    if u.len() <= 0 {
        return Vector::<f64>::new();
    }

    let mut v = Vector::<f64>::from(vec![0.0f64; u[0].len()]);

    for i in 0..u.len() {
        let mut uv = u[i].clone();
        uv.scl(coefs[i]);
        v.add(&uv);
    }

    v
}

#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::ex01::linear_combination;

    #[test]
    fn vector() {
        let e1 = Vector::<f64>::from(vec![1.0f64, 0.0f64, 0.0f64]);
        let e2 = Vector::<f64>::from(vec![0.0f64, 1.0f64, 0.0f64]);
        let e3 = Vector::<f64>::from(vec![0.0f64, 0.0f64, 1.0f64]);
        let v1 = Vector::<f64>::from(vec![1.0f64, 2.0f64, 3.0f64]);
        let v2 = Vector::<f64>::from(vec![0.0f64, 10.0f64, -100.0f64]);
        
        println!("Linear Combination: {}", linear_combination(&[e1, e2, e3], &[10.0f64, -2.0f64, 0.5f64]));
    }
}
