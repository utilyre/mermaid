use std::ops::{Add, AddAssign};

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

pub struct RowIter<'a, T, const COLS: usize> {
    entries: &'a [T],
    index: usize,
    end: usize,
}

impl<'a, T, const COLS: usize> RowIter<'a, T, COLS> {
    fn new(entries: &'a [T], i: usize) -> Self {
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
    fn new(entries: &'a [T], j: usize) -> Self {
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
    use super::*;

    #[test]
    #[should_panic(expected = "given `entries` do not match matrix size")]
    fn instantiation() {
        Matrix::<_, 2, 3>::new(vec![0u32, 2]);
    }

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
