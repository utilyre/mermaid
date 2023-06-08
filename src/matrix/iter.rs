use super::Matrix;

pub struct RowIter<'m, T, const M: usize, const N: usize> {
    matrix: &'m Matrix<T, M, N>,
    index: usize,
    end: usize,
}

impl<'m, T, const M: usize, const N: usize> RowIter<'m, T, M, N> {
    pub(super) fn new(matrix: &'m Matrix<T, M, N>, i: usize) -> Self {
        Self {
            matrix,
            index: N * i,
            end: N * (i + 1) - 1,
        }
    }
}

impl<'m, T, const M: usize, const N: usize> Iterator for RowIter<'m, T, M, N> {
    type Item = &'m T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index <= self.end).then(|| {
            let idx = self.index;
            self.index += 1;

            &self.matrix.0[idx]
        })
    }
}

pub struct ColIter<'m, T, const M: usize, const N: usize> {
    matrix: &'m Matrix<T, M, N>,
    index: usize,
    end: usize,
}

impl<'m, T, const M: usize, const N: usize> ColIter<'m, T, M, N> {
    pub(super) fn new(matrix: &'m Matrix<T, M, N>, j: usize) -> Self {
        Self {
            matrix,
            index: j,
            end: (M - 1) * N + j,
        }
    }
}

impl<'m, T, const M: usize, const N: usize> Iterator for ColIter<'m, T, M, N> {
    type Item = &'m T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index <= self.end).then(|| {
            let idx = self.index;
            self.index += N;

            &self.matrix.0[idx]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Matrix;
    use std::error::Error;

    #[test]
    fn row_iter() -> Result<(), Box<dyn Error>> {
        let m: Matrix<_, 5, 3> = Matrix::new(
            vec![0u32; 15]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + 1)
                .collect(),
        )?;

        let mut iter = m.row_iter(2).expect("row exists");
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn col_iter() -> Result<(), Box<dyn Error>> {
        let m: Matrix<_, 5, 3> = Matrix::new(
            vec![0u32; 15]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + 1)
                .collect(),
        )?;

        let mut iter = m.col_iter(1).expect("col exists");
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), Some(&14));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        Ok(())
    }
}
