pub mod iter;
pub mod ops;

use crate::error::{Error, Result};
use iter::{ColIter, RowIter};

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    entries: Vec<T>,
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(entries: Vec<T>) -> Result<Self> {
        if entries.len() != M * N {
            return Err(Error::WrongEntriesLength {
                rows: M,
                cols: N,
                len: entries.len(),
            });
        }

        Ok(Self { entries })
    }

    pub const fn is_empty(&self) -> bool {
        M == 0 || N == 0
    }

    pub const fn is_row(&self) -> bool {
        M == 1
    }

    pub const fn is_col(&self) -> bool {
        N == 1
    }

    pub const fn is_square(&self) -> bool {
        M == N
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.entries.get(N * i + j)
    }

    pub fn iter_row(&self, i: usize) -> Option<RowIter<T, N>> {
        (i < M).then_some(RowIter::new(&self.entries, i))
    }

    pub fn iter_col(&self, j: usize) -> Option<ColIter<T, M, N>> {
        (j < N).then_some(ColIter::new(&self.entries, j))
    }
}

impl<T, const M: usize, const N: usize> Eq for Matrix<T, M, N> where T: Eq {}

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N>
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
    #[should_panic = "entry array of size `2` does not match matrix of size `2x3`"]
    fn instantiation() {
        if let Err(err) = Matrix::<_, 2, 3>::new(vec![0u32, 2]) {
            panic!("{}", err);
        }
    }
}
