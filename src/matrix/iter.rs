pub struct RowIter<'a, T, const COLS: usize> {
    entries: &'a [T],
    index: usize,
    end: usize,
}

impl<'a, T, const COLS: usize> RowIter<'a, T, COLS> {
    pub(super) fn new(entries: &'a [T], i: usize) -> Self {
        Self {
            entries,
            index: COLS * i,
            end: COLS * (i + 1) - 1,
        }
    }
}

impl<'a, T, const COLS: usize> Iterator for RowIter<'a, T, COLS> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index <= self.end).then(|| {
            let idx = self.index;
            self.index += 1;

            &self.entries[idx]
        })
    }
}

pub struct ColIter<'a, T, const ROWS: usize, const COLS: usize> {
    entries: &'a [T],
    index: usize,
    end: usize,
}

impl<'a, T, const ROWS: usize, const COLS: usize> ColIter<'a, T, ROWS, COLS> {
    pub(super) fn new(entries: &'a [T], j: usize) -> Self {
        Self {
            entries,
            index: j,
            end: (ROWS - 1) * COLS + j,
        }
    }
}

impl<'a, T, const ROWS: usize, const COLS: usize> Iterator for ColIter<'a, T, ROWS, COLS> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.index <= self.end).then(|| {
            let idx = self.index;
            self.index += COLS;

            &self.entries[idx]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::Matrix;

    #[test]
    fn iter_row() {
        let m: Matrix<_, 5, 3> = Matrix::new(
            vec![0u32; 15]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + 1)
                .collect(),
        );

        let mut iter = m.iter_row(2).unwrap();
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_col() {
        let m: Matrix<_, 5, 3> = Matrix::new(
            vec![0u32; 15]
                .into_iter()
                .enumerate()
                .map(|(i, _)| i + 1)
                .collect(),
        );

        let mut iter = m.iter_col(1).unwrap();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), Some(&14));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
