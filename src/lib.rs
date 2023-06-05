#[derive(Debug)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    entries: Box<[T]>,
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new(entries: Box<[T]>) -> Self {
        Self { entries }
    }

    pub fn with_vec(entries: Vec<T>) -> Self {
        Self::new(entries.into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let m1: Matrix<_, 2, 5> = Matrix::new(Box::new([0u32; 10]));
        let m2: Matrix<_, 5, 2> = Matrix::with_vec(vec![0u32; 10]);
        assert_eq!(m1.entries, m2.entries);
    }
}
