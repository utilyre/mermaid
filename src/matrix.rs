pub mod iter;
pub mod ops;

use crate::error::{Error, Result};
use iter::{ColIter, RowIter};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>(Vec<T>);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(entries: Vec<T>) -> Result<Self> {
        if entries.len() != M * N {
            return Err(Error::WrongEntriesLength {
                rows: M,
                cols: N,
                len: entries.len(),
            });
        }

        Ok(Self(entries))
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
        self.0.get(N * i + j)
    }

    pub fn row_iter(&self, i: usize) -> Option<RowIter<T, N>> {
        (i < M).then_some(RowIter::new(&self.0, i))
    }

    pub fn col_iter(&self, j: usize) -> Option<ColIter<T, M, N>> {
        (j < N).then_some(ColIter::new(&self.0, j))
    }
}

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let entries: Vec<_> = self.0.iter().map(|entry| entry.to_string()).collect();
        let Some(longest) = entries.iter().map(|entry| entry.len()).max() else {
            return write!(f, "||");
        };

        for i in 0..M {
            write!(
                f,
                "|{}|",
                &entries[N * i..N * (i + 1)]
                    .iter()
                    .fold(String::new(), |acc, entry| {
                        acc + &format!("{: ^longest$}", entry, longest = longest + 2)
                    })
            )?;

            if i < M - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<T, const M: usize, const N: usize> Eq for Matrix<T, M, N> where T: Eq {}

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
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
