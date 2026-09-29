use crate::Vector;

pub fn angle_cos(u: &Vector<f32>, v: &Vector<f32>) -> f32 {
    u.dot(v) / (u.norm() * v.norm())
}

#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::ex05::angle_cos;

    #[test]
    fn test() {
        let u = Vector::<f32>::from(vec![1.0, 0.0]);
        let v = Vector::<f32>::from(vec![1.0, 0.0]);
        assert_eq!(angle_cos(&u, &v), 1.0);

        let u = Vector::<f32>::from(vec![1.0, 0.0]);
        let v = Vector::<f32>::from(vec![0.0, 1.0]);
        assert_eq!(angle_cos(&u, &v), 0.0);

        let u = Vector::<f32>::from(vec![-1.0, 1.0]);
        let v = Vector::<f32>::from(vec![1.0, -1.0]);
        println!("Cos: {}", angle_cos(&u, &v));
        assert!((angle_cos(&u, &v) + 1.0).abs() < 1e-6);

        let u = Vector::<f32>::from(vec![1.0, 2.0]);
        let v = Vector::<f32>::from(vec![3.0, 4.0]);

        assert!((angle_cos(&u, &v) - 0.9838699).abs() < 1e-6);
    }
}
