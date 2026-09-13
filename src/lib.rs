pub mod ex00;

struct Vector::<K> {
    data: Vec<K>,
}

struct Matrix::<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {Vec::new(), rows, cols}
    }

    fn index(&self, x: u16, y: u16) -> u16 {
        x + y * self.rows
    } 
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
