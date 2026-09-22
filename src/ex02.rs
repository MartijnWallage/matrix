use crate::Vector;

/* Linear interpolation of u and v:
 * u: first vector
 * v: second vector
 * t: scalar weight (should be f32)
 * result: (1-t)u + t*v
 */
pub fn lerp(u: Vector<f64>, v: Vector<f64>, t: f64) -> Vector<f64> {
    let mut result = u.clone();
    let mut v_cpy = v.clone();

    result.scl(1f64-t);
    v_cpy.scl(t);

    result.add(&v_cpy);

    result
}

#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::ex02::lerp;

    #[test]
    fn test_lerp() {
        let u = Vector::<f64>::from(vec![2f64, 1f64]);
        let v = Vector::<f64>::from(vec![4f64, 2f64]);

        let result = lerp(u, v, 0.3);
        println!("Result of lerp: {}", result);
        assert_eq!(result, Vector::<f64>::from(vec![2.6, 1.3]));
    }
}
