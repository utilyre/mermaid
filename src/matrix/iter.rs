use super::Matrix;
use std::ptr;

pub mod cursor;

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn iter(&self) -> Iter<T, M, N> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T, M, N> {
        self.into_iter()
    }
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a Matrix<T, M, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.rows())
    }
}

pub struct Iter<'a, T, const M: usize, const N: usize>
where
    T: 'a,
{
    rows: [[&'a T; N]; M],
    i: usize,
    j: usize,
}

impl<'a, T, const M: usize, const N: usize> Iter<'a, T, M, N> {
    fn new(rows: [[&'a T; N]; M]) -> Self {
        Self { rows, i: 0, j: 0 }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for Iter<'a, T, M, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j == N {
            if self.i == M - 1 {
                return None;
            }

            self.i += 1;
            self.j = 0;
        }

        self.j += 1;
        Some(self.rows[self.i][self.j - 1])
    }
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a mut Matrix<T, M, N> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.rows_mut())
    }
}

pub struct IterMut<'a, T, const M: usize, const N: usize>
where
    T: 'a,
{
    rows: [[&'a mut T; N]; M],
    i: usize,
    j: usize,
}

impl<'a, T, const M: usize, const N: usize> IterMut<'a, T, M, N> {
    fn new(rows: [[&'a mut T; N]; M]) -> Self {
        Self { rows, i: 0, j: 0 }
    }
}

impl<'a, T, const M: usize, const N: usize> Iterator for IterMut<'a, T, M, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j == N {
            if self.i == M - 1 {
                return None;
            }

            self.i += 1;
            self.j = 0;
        }

        self.j += 1;
        Some(unsafe { &mut *(self.rows[self.i][self.j - 1] as *mut T) })
    }
}

impl<T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N> {
    type Item = T;
    type IntoIter = IntoIter<T, M, N>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.into_rows())
    }
}

pub struct IntoIter<T, const M: usize, const N: usize> {
    rows: [[T; N]; M],
    i: usize,
    j: usize,
}

impl<T, const M: usize, const N: usize> IntoIter<T, M, N> {
    fn new(rows: [[T; N]; M]) -> Self {
        Self { rows, i: 0, j: 0 }
    }
}

impl<T, const M: usize, const N: usize> Iterator for IntoIter<T, M, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j == N {
            if self.i == M - 1 {
                return None;
            }

            self.i += 1;
            self.j = 0;
        }

        self.j += 1;
        Some(unsafe { ptr::read(&self.rows[self.i][self.j - 1]) })
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

    #[test]
    fn iter_mut() {
        let mut mat = Matrix::new([[1, 2], [3, 4], [5, 6]]);

        let mut iter = mat.iter_mut();
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(Some(&mut 2), iter.next());
        assert_eq!(Some(&mut 3), iter.next());
        assert_eq!(Some(&mut 4), iter.next());
        assert_eq!(Some(&mut 5), iter.next());
        assert_eq!(Some(&mut 6), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iter() {
        let mat = Matrix::new([[1, 2], [3, 4], [5, 6]]);

        let mut iter = mat.into_iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(5), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
