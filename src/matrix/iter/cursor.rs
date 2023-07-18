use super::{IntoIter, Iter, IterMut};

pub struct Cursor<I, const M: usize, const N: usize>
where
    I: CursorIterator<M, N>,
{
    iter: I,
    i: usize,
    j: usize,
}

impl<I, const M: usize, const N: usize> Cursor<I, M, N>
where
    I: CursorIterator<M, N>,
{
    fn new(iter: I) -> Self {
        Self { iter, i: 0, j: 0 }
    }
}

impl<I, const M: usize, const N: usize> Iterator for Cursor<I, M, N>
where
    I: CursorIterator<M, N>,
{
    type Item = (usize, usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.j == N {
            self.i += 1;
            self.j = 0;
        }

        let j = self.j;
        self.j += 1;

        self.iter.next().map(|x| (self.i, j, x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = N * (M - self.i) - self.j;
        (size, Some(size))
    }
}

impl<I, const M: usize, const N: usize> ExactSizeIterator for Cursor<I, M, N> where
    I: CursorIterator<M, N>
{
}

pub trait CursorIterator<const M: usize, const N: usize>: Sized + Iterator {
    fn cursor(self) -> Cursor<Self, M, N>;
}

impl<'a, T, const M: usize, const N: usize> CursorIterator<M, N> for Iter<'a, T, M, N> {
    fn cursor(self) -> Cursor<Self, M, N> {
        Cursor::new(self)
    }
}

impl<'a, T, const M: usize, const N: usize> CursorIterator<M, N> for IterMut<'a, T, M, N> {
    fn cursor(self) -> Cursor<Self, M, N> {
        Cursor::new(self)
    }
}

impl<T, const M: usize, const N: usize> CursorIterator<M, N> for IntoIter<T, M, N> {
    fn cursor(self) -> Cursor<Self, M, N> {
        Cursor::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Matrix;
    use super::*;

    #[test]
    fn cursor() {
        let mat = Matrix::from_rows([[1, 2], [3, 4], [5, 6]]);

        let mut iter = mat.iter().cursor();
        assert_eq!(6, iter.len());
        assert_eq!(Some((0, 0, &1)), iter.next());
        assert_eq!(5, iter.len());
        assert_eq!(Some((0, 1, &2)), iter.next());
        assert_eq!(4, iter.len());
        assert_eq!(Some((1, 0, &3)), iter.next());
        assert_eq!(3, iter.len());
        assert_eq!(Some((1, 1, &4)), iter.next());
        assert_eq!(2, iter.len());
        assert_eq!(Some((2, 0, &5)), iter.next());
        assert_eq!(1, iter.len());
        assert_eq!(Some((2, 1, &6)), iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
        assert_eq!(0, iter.len());
        assert_eq!(None, iter.next());
    }
}
