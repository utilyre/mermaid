use std::ops::{Add, AddAssign};

#[derive(Debug, Eq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    entries: Vec<T>,
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new(entries: Vec<T>) -> Self {
        // TODO: Consider an `Error` type that could be returned via `Result`
        assert_eq!(
            entries.len(),
            ROWS * COLS,
            "given `entries` do not match matrix size"
        );

        Self { entries }
    }

    pub const fn is_empty(&self) -> bool {
        ROWS == 0 || COLS == 0
    }

    pub const fn is_row(&self) -> bool {
        ROWS == 1
    }

    pub const fn is_col(&self) -> bool {
        COLS == 1
    }

    pub const fn is_square(&self) -> bool {
        ROWS == COLS
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.entries.get(COLS * i + j)
    }
}

impl<T, const ROWS: usize, const COLS: usize> PartialEq for Matrix<T, ROWS, COLS>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(a, b)| a == b)
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.entries
                .into_iter()
                .zip(rhs.entries.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl<T, const ROWS: usize, const COLS: usize> AddAssign for Matrix<T, ROWS, COLS>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.entries
            .iter_mut()
            .zip(rhs.entries.into_iter())
            .for_each(|(a, b)| {
                *a += b;
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "given `entries` do not match matrix size")]
    fn instantiation() {
        Matrix::<_, 2, 3>::new(vec![0u32, 2]);
    }

    #[test]
    fn addition() {
        let m1: Matrix<_, 2, 5> = Matrix::new(vec![5u32; 10]);
        let m2: Matrix<_, 2, 5> = Matrix::new(vec![2u32; 10]);

        let expected = Matrix::new(vec![7u32; 10]);
        let actual = m1 + m2;
        assert_eq!(expected, actual);

        let mut m1: Matrix<_, 3, 7> = Matrix::new(vec![9i32; 21]);
        let m2: Matrix<_, 3, 7> = Matrix::new(vec![1i32; 21]);

        let expected = Matrix::new(vec![10i32; 21]);
        m1 += m2;
        assert_eq!(expected, m1);
    }
}
