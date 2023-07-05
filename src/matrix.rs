pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn with_rows(rows: [[T; N]; M]) -> Self {
        Self(rows)
    }

    pub fn row(&self, i: usize) -> Option<[&T; N]> {
        self.0.get(i).map(|row| row.each_ref())
    }

    pub fn col(&self, j: usize) -> Option<[&T; M]> {
        (j < N).then(|| self.0.each_ref().map(|row| &row[j]))
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
    fn col() {
        let m = matrix_4x3();

        assert_eq!(Some([&1, &4, &7, &10]), m.col(0));
        assert_eq!(Some([&2, &5, &8, &11]), m.col(1));
        assert_eq!(Some([&3, &6, &9, &12]), m.col(2));
        assert_eq!(None, m.col(3));
        assert_eq!(None, m.col(4));
    }
}
