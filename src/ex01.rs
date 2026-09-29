use crate::Vector;

pub fn linear_combination(u: &[Vector<f32>], coefs: &[f32]) -> Vector<f32> {
    if u.is_empty() {
        return Vector::<f32>::new();
    }

    let dim = u[0].len(); // assume the length of the first vector is the length of each vector
    let mut result = Vector::<f32>::from(vec![0.0; dim]);

    for j in 0..dim {
        let mut acc = 0.0;
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
        let e1 = Vector::<f32>::from(vec![1.0, 0.0, 0.0]);
        let e2 = Vector::<f32>::from(vec![0.0, 1.0, 0.0]);
        let e3 = Vector::<f32>::from(vec![0.0, 0.0, 1.0]);
        let v1 = Vector::<f32>::from(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::<f32>::from(vec![0.0, 10.0, -100.0]);
        
        let result_e = linear_combination(&[e1, e2, e3], &[10.0, -2.0, 0.5]);
        assert_eq!(result_e, Vector::from(vec![10.0, -2.0, 0.5]));
        let result_v = linear_combination(&[v1, v2], &[10.0, -2.0]);
        assert_eq!(result_v, Vector::from(vec![10.0, 0.0, 230.0]));
    }
}
