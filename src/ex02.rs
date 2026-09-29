use crate::Vector;
use crate::Matrix;

pub trait VectorSpace {
    fn add(&mut self, other: &Self);
    fn scl(&mut self, k: f32);
}

impl VectorSpace for Vector<f32> {
    fn add(&mut self, other: &Self) {
        self.add(other);
    }

    fn scl(&mut self, k: f32) {
        self.scl(k);
    }
}

impl VectorSpace for Matrix<f32> {
    fn add(&mut self, other: &Self) {
        self.add(other);
    }

    fn scl(&mut self, k: f32) {
        self.scl(k);
    }
}

impl VectorSpace for f32 {
    fn add(&mut self, other: &Self) {
        *self += other;
    }

    fn scl(&mut self, k: f32) {
        *self *= k;
    }
}

/* Linear interpolation of u and v:
 * u: first vector
 * v: second vector
 * t: scalar weight (should be f32)
 * result: (1-t)u + t*v
 */
pub fn lerp<V: VectorSpace + Clone>(u: V, v: V, t: f32) -> V {
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
        assert_eq!(lerp(0.0, 1.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 1.0, 1.0), 1.0);
        assert_eq!(lerp(0.0, 1.0, 0.3), 0.3);
    }

    #[test]
    fn test_vector() {
        let u = Vector::from(vec![2.0, 1.0]);
        let v = Vector::from(vec![4.0, 2.0]);

        let result = lerp::<Vector<f32>>(u, v, 0.3);
        println!("Result of lerp: {}", result);
        assert!((result.get(0) - 2.6).abs() < 1e-6);
        assert!((result.get(1) - 1.3).abs() < 1e-6);
    }

    #[test]
    fn test_matrix() {
        let m = Matrix::<f32>::from(vec![
            vec![2.0, 1.0],
            vec![3.0, 4.0]
        ]);
        let n = Matrix::<f32>::from(vec![
            vec![20.0, 10.0],
            vec![30.0, 40.0]
        ]);

        println!("Lerp matrices: {}", lerp(m, n, 0.5));
    }
}
