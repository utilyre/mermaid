pub mod iter;
pub mod ops;

use self::iter::{ColIter, RowIter};

// TODO: implement Debug manually
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

    pub fn iter_row(&self, i: usize) -> Option<RowIter<T, COLS>> {
        (i < ROWS).then_some(RowIter::new(&self.entries, i))
    }

    pub fn iter_col(&self, j: usize) -> Option<ColIter<T, ROWS, COLS>> {
        (j < COLS).then_some(ColIter::new(&self.entries, j))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "given `entries` do not match matrix size")]
    fn instantiation() {
        Matrix::<_, 2, 3>::new(vec![0u32, 2]);
    }
}
