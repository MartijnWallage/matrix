pub mod ex00;

#[derive(Debug, Clone)]
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

    pub fn get(&self, i: usize) -> &K {
        &self.data[i]
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

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

impl<K> Matrix<K> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {data: Vec::new(), rows, cols}
    }

    pub fn get(&self, x: usize, y: usize) -> &K {
        let index = x * self.cols + y;
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = Vector::<f64>::new();

        assert_eq!(v.len(), 0);
        v.push(1.0);
        v.push(2.0);
        v.push(3.0);
        assert_eq!(v.len(), 3);
        assert_eq!(*v.get(1), 2.0);
    }
}
