use std::fmt;

pub mod ex00;
pub mod ex01;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<K> {
    data: Vec<K>,
}

impl<K> Vector<K> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, i: usize) -> &K {
        &self.data[i]
    }

    pub fn set(&mut self, i: usize, a: K) {
        self.data[i] = a;
    }

    pub fn push(&mut self, new_data: K) {
        self.data.push(new_data);
    }
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(data: Vec<K>) -> Self {
        Self { data }
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in &self.data {
            writeln!(f, "[{}]", x)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

impl<K: Clone> Matrix<K> {
    pub fn new(rows: usize, cols: usize, data: K) -> Self {
        Self {
            data: vec![data; rows * cols],
            rows,
            cols,
        }
    }
}

impl<K> Matrix<K> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn get(&self, col: usize, row: usize) -> &K {
        let index = col * self.rows + row;
        &self.data[index]
    }

    pub fn set(&mut self, col: usize, row: usize, value: K) {
        let index = col * self.rows + row;
        self.data[index] = value;
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.rows {
            write!(f, "[")?;
            for x in 0..self.cols {
                if x > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        let mut v = Vector::<f64>::new();

        assert_eq!(v.len(), 0);
        assert!(v.is_empty());

        v.push(1.0);
        v.push(2.0);
        v.push(3.0);
        assert_eq!(v.len(), 3);
        assert_eq!(*v.get(0), 1.0);
        assert_eq!(*v.get(1), 2.0);
        assert_eq!(*v.get(2), 3.0);

        let vec = vec![0.1,0.2,0.3];
        let v = Vector::<f64>::from(vec);
        assert_eq!(v.len(), 3);
        assert_eq!(*v.get(0), 0.1);
        assert_eq!(*v.get(1), 0.2);
        assert_eq!(*v.get(2), 0.3);

        println!("{}", v);
    }

    #[test]
    fn matrix() {
        let mut m = Matrix::<f64>::new(3, 3, 0.0);

        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 3);
        assert_eq!(m.size(), 9);
        assert!(m.is_square());

        assert_eq!(*m.get(0,0), 0.0);
        assert_eq!(*m.get(1,1), 0.0);
        assert_eq!(*m.get(2,2), 0.0);

        m.set(0, 0, 1.0);
        m.set(2, 1, 2.0);
        m.set(1, 2, 3.0);

        assert_eq!(
            m,
            Matrix::<f64> {
                data: vec![1.0, 0.0, 0.0,
                     0.0, 0.0, 3.0,
                     0.0, 2.0, 0.0,
                ],
                rows: 3,
                cols: 3,
            });
        
        println!("{}", m);
    }
}
