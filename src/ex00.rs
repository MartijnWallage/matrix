use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;

use crate::Vector;
use crate::Matrix;

impl<K: AddAssign + SubAssign + MulAssign + Copy> Vector<K> {
    fn add(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() {
            self.data[i] += v.data[i];
        }
    }

    fn sub(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() {
            self.data[i] -= v.data[i];
        }
    }

    fn scl(&mut self, a: K) {
        for i in 0..self.data.len() {
            self.data[i] *= a;
        }
    }
}


impl<K: AddAssign + SubAssign + MulAssign + Copy> Matrix<K> {
    fn add(&mut self, v: &Matrix<K>) {
        for i in 0..self.size() {
            self.data[i] += v.data[i];
        }
    }

    fn sub(&mut self, v: &Matrix<K>) {
        for i in 0..self.size() {
            self.data[i] -= v.data[i];
        }
    }

    fn scl(&mut self, a: K) {
        for i in 0..self.size() {
            self.data[i] *= a;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;
    use crate::Matrix;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    fn vector_approx_eq(u: &Vector::<f64>, v: &Vector::<f64>) -> bool {
        if u.len() != v.len() {
            return false
        }

        for i in 0..u.len() {
            if !approx_eq(*u.get(i), *v.get(i)) {
                return false
            }
        }

        true
    }

    #[test]
    fn vector() {
        let mut v = Vector::<f64>::from(vec![0.1, 0.2, 0.3]);
        let u = Vector::<f64>::from(vec![0.9, 0.8, 0.7]);

        v.add(&u);
        println!("Add: {}", v);
        assert_eq!(v, Vector::<f64>::from(vec![1., 1., 1.]));

        v.sub(&u);

        println!("Sub: {}", v);
        assert!(vector_approx_eq(&v, &Vector::<f64>::from(vec![0.1, 0.2, 0.3])));

        v.scl(10.);
        assert!(vector_approx_eq(&v, &Vector::<f64>::from(vec![1., 2., 3.])));
    }

    #[test]
    fn matrix() {
        let mut m = Matrix::<f64>::new(3, 3, 0.0);
        let u = Matrix::<f64>::new(3, 3, 0.1);

        m.set(0, 0, 1.0);
        m.set(2, 1, 2.0);
        m.set(1, 2, 3.0);
        m.set(2, 0, 4.0);
        m.set(0, 2, 5.0);

        println!("m: {}", m);
        println!("u: {}", u);

        m.add(&u);
        println!("Add: {}", m);
        m.sub(&u);
        println!("Sub: {}", m);
        m.scl(10.0);
        println!("Scl: {}", m);
    }
}
