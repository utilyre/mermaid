use super::Matrix;

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T, const M: usize, const N: usize> Eq for Matrix<T, M, N> where T: Eq {}
