pub struct RowIter<'e, T, const N: usize> {
    entries: &'e [T],
    index: usize,
    end: usize,
}

impl<'e, T, const N: usize> RowIter<'e, T, N> {
    pub(super) fn new(entries: &'e [T], i: usize) -> Self {
        Self {
            entries,
            index: N * i,
            end: N * (i + 1) - 1,
        }
    }
}

impl<'e, T, const N: usize> Iterator for RowIter<'e, T, N> {
    type Item = &'e T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index <= self.end).then(|| {
            let idx = self.index;
            self.index += 1;

            &self.entries[idx]
        })
    }
}

pub struct ColIter<'e, T, const M: usize, const N: usize> {
    entries: &'e [T],
    index: usize,
    end: usize,
}

impl<'e, T, const M: usize, const N: usize> ColIter<'e, T, M, N> {
    pub(super) fn new(entries: &'e [T], j: usize) -> Self {
        Self {
            entries,
            index: j,
            end: (M - 1) * N + j,
        }
    }
}

impl<'e, T, const M: usize, const N: usize> Iterator for ColIter<'e, T, M, N> {
    type Item = &'e T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index <= self.end).then(|| {
            let idx = self.index;
            self.index += N;

            &self.entries[idx]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Matrix;
    use std::error::Error;

    #[test]
    fn iter_row() -> Result<(), Box<dyn Error>> {
        let m: Matrix<_, 5, 3> = Matrix::new(
            vec![0u32; 15]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + 1)
                .collect(),
        )?;

        let mut iter = m.iter_row(2).expect("row exists");
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn iter_col() -> Result<(), Box<dyn Error>> {
        let m: Matrix<_, 5, 3> = Matrix::new(
            vec![0u32; 15]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + 1)
                .collect(),
        )?;

        let mut iter = m.iter_col(1).expect("col exists");
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
