use crate::Vector;

/*
 * u x v = (
 *  u_y * v_z - u_z * v_y
 *  u_z * v_x - u_x * v_z
 *  u_x * v_y - u_y * v_x
 *  )
 */
pub fn cross_product(u: &Vector<f32>, v: &Vector<f32>) -> Vector<f32> {
    Vector::<f32>::from(vec![
        u.get(1) * v.get(2) - u.get(2) * v.get(1),
        u.get(2) * v.get(0) - u.get(0) * v.get(2),
        u.get(0) * v.get(1) - u.get(1) * v.get(0),
    ])
}

#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::ex06::cross_product;

    #[test]
    fn cross() {
        let u = Vector::<f32>::from(vec![0., 0., 1.]);
        let v = Vector::<f32>::from(vec![1., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::<f32>::from(vec![0.,1.,0.]));

        let u = Vector::<f32>::from(vec![1., 2., 3.]);
        let v = Vector::<f32>::from(vec![4., 5., 6.]);
        assert_eq!(cross_product(&u, &v), Vector::<f32>::from(vec![-3.,6.,-3.]));

        let u = Vector::<f32>::from(vec![4., 2., -3.]);
        let v = Vector::<f32>::from(vec![-2., -5., 16.]);
        assert_eq!(cross_product(&u, &v), Vector::<f32>::from(vec![17., -58., -16.]));
    }
}
