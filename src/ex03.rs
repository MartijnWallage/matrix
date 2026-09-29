use crate::Vector;

impl Vector<f32> {
    pub fn dot(&self, v: &Vector<f32>) -> f32 {
        let mut acc = 0.0;

        for i in 0..self.len() {
            acc = (*self.get(i))
                .mul_add(*v.get(i), acc);
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_dot() {
        let u = Vector::<f32>::from(vec![1.0, 2.0, 3.0]);
        let v = Vector::<f32>::from(vec![0.1, 0.2, 0.3]);

        println!("Dot u*v: {}", u.dot(&v));
    }
}
