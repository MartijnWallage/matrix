use crate::Vector;

pub fn linear_combination(u: &[Vector<f64>], coefs: &[f64]) -> Vector<f64> {
    if u.is_empty() {
        return Vector::<f64>::new();
    }

    let mut result = Vector::<f64>::from(vec![0.0f64; u[0].len()]);

    // for each place in the vectors (the first vector)
    // multiply by corresponding coefficient and add together
    for i in 0..u[0].len() {
        let mut acc = 0.0f64;
        for j in 0..u.len() {
            acc = coefs[j].mul_add(*u[j].get(i), acc);
            println!("Acc: {}", acc);
        }
        result.set(i, acc);
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
        
        println!("Linear Combination e: {}", linear_combination(&[e1, e2, e3], &[10.0f64, -2.0f64, 0.5f64]));
        println!("Linear Combination v: {}", linear_combination(&[v1, v2], &[10.0f64, -2.0f64]));
    }
}
