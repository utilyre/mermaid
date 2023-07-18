use super::{IntoIter, Iter, IterMut};

pub struct Cursor<I, const N: usize>
where
    I: CursorIterator<N>,
{
    iter: I,
    i: usize,
    j: usize,
}

impl<I, const N: usize> Cursor<I, N>
where
    I: CursorIterator<N>,
{
    fn new(iter: I) -> Self {
        Self { iter, i: 0, j: 0 }
    }
}

impl<I, const N: usize> Iterator for Cursor<I, N>
where
    I: CursorIterator<N>,
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
}

pub trait CursorIterator<const N: usize>: Sized + Iterator {
    fn cursor(self) -> Cursor<Self, N>;
}

impl<'a, T, const M: usize, const N: usize> CursorIterator<N> for Iter<'a, T, M, N> {
    fn cursor(self) -> Cursor<Self, N> {
        Cursor::new(self)
    }
}

impl<'a, T, const M: usize, const N: usize> CursorIterator<N> for IterMut<'a, T, M, N> {
    fn cursor(self) -> Cursor<Self, N> {
        Cursor::new(self)
    }
}

impl<T, const M: usize, const N: usize> CursorIterator<N> for IntoIter<T, M, N> {
    fn cursor(self) -> Cursor<Self, N> {
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
        assert_eq!(Some((0, 0, &1)), iter.next());
        assert_eq!(Some((0, 1, &2)), iter.next());
        assert_eq!(Some((1, 0, &3)), iter.next());
        assert_eq!(Some((1, 1, &4)), iter.next());
        assert_eq!(Some((2, 0, &5)), iter.next());
        assert_eq!(Some((2, 1, &6)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
