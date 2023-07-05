#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn with_rows(rows: [[T; N]; M]) -> Self {
        Self(rows)
    }

    pub const fn with_cols(_cols: [[T; M]; N]) -> Self {
        todo!()
    }

    pub fn row(&self, i: usize) -> Option<[&T; N]> {
        self.0.get(i).map(|row| row.each_ref())
    }

    pub fn row_mut(&mut self, i: usize) -> Option<[&mut T; N]> {
        self.0.get_mut(i).map(|row| row.each_mut())
    }

    pub fn col(&self, j: usize) -> Option<[&T; M]> {
        (j < N).then(|| self.0.each_ref().map(|row| &row[j]))
    }

    pub fn col_mut(&mut self, j: usize) -> Option<[&mut T; M]> {
        (j < N).then(|| self.0.each_mut().map(|row| &mut row[j]))
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.0.get(i).and_then(|row| row.get(j))
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.0.get_mut(i).and_then(|row| row.get_mut(j))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn matrix_4x3() -> Matrix<i32, 4, 3> {
        Matrix::with_rows([
            [1,  2,  3 ],
            [4,  5,  6 ],
            [7,  8,  9 ],
            [10, 11, 12],
        ])
    }

    #[test]
    fn row() {
        let m = matrix_4x3();

        assert_eq!(Some([&1, &2, &3]), m.row(0));
        assert_eq!(Some([&4, &5, &6]), m.row(1));
        assert_eq!(Some([&7, &8, &9]), m.row(2));
        assert_eq!(Some([&10, &11, &12]), m.row(3));
        assert_eq!(None, m.row(4));
        assert_eq!(None, m.row(5));
    }

    #[test]
    fn row_mut() {
        let mut m = matrix_4x3();

        assert_eq!(Some([&mut 1, &mut 2, &mut 3]), m.row_mut(0));
        assert_eq!(Some([&mut 4, &mut 5, &mut 6]), m.row_mut(1));
        assert_eq!(Some([&mut 7, &mut 8, &mut 9]), m.row_mut(2));
        assert_eq!(Some([&mut 10, &mut 11, &mut 12]), m.row_mut(3));
        assert_eq!(None, m.row_mut(4));
        assert_eq!(None, m.row_mut(5));
    }

    #[test]
    fn col() {
        let m = matrix_4x3();

        assert_eq!(Some([&1, &4, &7, &10]), m.col(0));
        assert_eq!(Some([&2, &5, &8, &11]), m.col(1));
        assert_eq!(Some([&3, &6, &9, &12]), m.col(2));
        assert_eq!(None, m.col(3));
        assert_eq!(None, m.col(4));
    }

    #[test]
    fn col_mut() {
        let mut m = matrix_4x3();

        assert_eq!(Some([&mut 1, &mut 4, &mut 7, &mut 10]), m.col_mut(0));
        assert_eq!(Some([&mut 2, &mut 5, &mut 8, &mut 11]), m.col_mut(1));
        assert_eq!(Some([&mut 3, &mut 6, &mut 9, &mut 12]), m.col_mut(2));
        assert_eq!(None, m.col_mut(3));
        assert_eq!(None, m.col_mut(4));
    }

    #[test]
    fn get() {
        let m = matrix_4x3();

        assert_eq!(Some(&1), m.get(0, 0));
        assert_eq!(Some(&4), m.get(1, 0));
        assert_eq!(Some(&12), m.get(3, 2));
        assert_eq!(None, m.get(4, 0));
        assert_eq!(None, m.get(2, 5));
        assert_eq!(None, m.get(6, 7));
    }

    #[test]
    fn get_mut() {
        let mut m = matrix_4x3();

        assert_eq!(Some(&mut 1), m.get_mut(0, 0));
        assert_eq!(Some(&mut 4), m.get_mut(1, 0));
        assert_eq!(Some(&mut 12), m.get_mut(3, 2));
        assert_eq!(None, m.get_mut(4, 0));
        assert_eq!(None, m.get_mut(2, 5));
        assert_eq!(None, m.get_mut(6, 7));
    }
}
