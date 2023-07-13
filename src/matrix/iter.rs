use super::Matrix;

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn iter(&self) -> Iter<T, M, N> {
        Iter::new(self.rows())
    }
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a Matrix<T, M, N> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T, const M: usize, const N: usize> {
    i: usize,
    j: usize,
    rows: &'a [[T; N]; M],
}

impl<'a, T, const M: usize, const N: usize> Iter<'a, T, M, N> {
    pub fn new(rows: &'a [[T; N]; M]) -> Self {
        Self { i: 0, j: 0, rows }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for Iter<'a, T, M, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= N {
            self.i += 1;
            if self.i >= M {
                return None;
            }

            self.j = 0;
        }

        self.j += 1;
        Some(&self.rows[self.i][self.j - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let mat = Matrix::new([[1, 2], [3, 4], [5, 6]]);

        let mut iter = mat.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&6), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
