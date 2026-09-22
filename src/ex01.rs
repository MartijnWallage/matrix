use crate::Vector;

pub fn linear_combination(u: &[Vector<f64>], coefs: &[f64]) -> Vector<f64> {
    if u.is_empty() {
        return Vector::<f64>::new();
    }

    let dim = u[0].len(); // assume the length of the first vector is the length of each vector
    let mut result = Vector::<f64>::from(vec![0.0f64; dim]);

    for j in 0..dim {
        let mut acc = 0.0f64;
        for i in 0..u.len() {
            let value = *u[i].get(j);
            acc = coefs[i].mul_add(value, acc);
        }
        result.set(j, acc);
    }

    result
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
        
        let result_e = linear_combination(&[e1, e2, e3], &[10.0f64, -2.0f64, 0.5f64]);
        assert_eq!(result_e, Vector::from(vec![10f64, -2f64, 0.5f64]));
        let result_v = linear_combination(&[v1, v2], &[10f64, -2f64]);
        assert_eq!(result_v, Vector::from(vec![10f64, 0f64, 230f64]));
    }
}
