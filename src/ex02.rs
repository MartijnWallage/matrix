use crate::Vector;
use crate::Matrix;

pub trait VectorSpace {
    fn add(&mut self, other: &Self);
    fn scl(&mut self, k: f64);
}

impl VectorSpace for Vector<f64> {
    fn add(&mut self, other: &Self) {
        self.add(other);
    }

    fn scl(&mut self, k: f64) {
        self.scl(k);
    }
}

impl VectorSpace for Matrix<f64> {
    fn add(&mut self, other: &Self) {
        self.add(other);
    }

    fn scl(&mut self, k: f64) {
        self.scl(k);
    }
}

impl VectorSpace for f64 {
    fn add(&mut self, other: &Self) {
        *self += other;
    }

    fn scl(&mut self, k: f64) {
        *self *= k;
    }
}

/* Linear interpolation of u and v:
 * u: first vector
 * v: second vector
 * t: scalar weight (should be f32)
 * result: (1-t)u + t*v
 */
pub fn lerp<V: VectorSpace + Clone>(u: V, v: V, t: f64) -> V {
    let mut result = u.clone();
    let mut v_cpy = v.clone();

    result.scl(1.0-t);
    v_cpy.scl(t);

    result.add(&v_cpy);

    result
}

#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::Matrix;
    use crate::ex02::lerp;

    #[test]
    fn test_scalar() {
        assert_eq!(lerp(0f64, 1f64, 0f64), 0f64);
        assert_eq!(lerp(0f64, 1f64, 1f64), 1f64);
        assert_eq!(lerp(0f64, 1f64, 0.3f64), 0.3f64);
    }

    #[test]
    fn test_vector() {
        let u = Vector::from(vec![2f64, 1f64]);
        let v = Vector::from(vec![4f64, 2f64]);

        let result = lerp::<Vector<f64>>(u, v, 0.3);
        println!("Result of lerp: {}", result);
        assert!((result.get(0) - 2.6).abs() < 1e-6);
        assert!((result.get(1) - 1.3).abs() < 1e-6);
    }

    #[test]
    fn test_matrix() {
        let m = Matrix::<f64>::from(vec![
            vec![2f64, 1f64],
            vec![3f64, 4f64]
        ]);
        let n = Matrix::<f64>::from(vec![
            vec![20f64, 10f64],
            vec![30f64, 40f64]
        ]);

        println!("Lerp matrices: {}", lerp(m, n, 0.5));
    }
}
