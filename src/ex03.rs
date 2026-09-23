use crate::Vector;

impl Vector<f64> {
    pub fn dot(&self, v: Vector<f64>) -> f64 {
        let mut acc = 0f64;

        for i in 0..self.len() {
            acc += *self.get(i) * *v.get(i);
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_dot() {
        let u = Vector::<f64>::from(vec![1.0, 2.0, 3.0]);
        let v = Vector::<f64>::from(vec![0.1, 0.2, 0.3]);

        println!("Dot u*v: {}", u.dot(v));
    }
}
