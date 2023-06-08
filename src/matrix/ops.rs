use super::Matrix;
use std::ops::{Add, AddAssign};

impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.0
                .into_iter()
                .zip(rhs.0.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
        .expect("entries is the same size as matrix")
    }
}

impl<T, const M: usize, const N: usize> AddAssign for Matrix<T, M, N>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0.into_iter()).for_each(|(a, b)| {
            *a += b;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::super::Matrix;
    use std::error::Error;

    #[test]
    fn addition() -> Result<(), Box<dyn Error>> {
        let m1: Matrix<_, 2, 5> = Matrix::new(vec![5u32; 10])?;
        let m2: Matrix<_, 2, 5> = Matrix::new(vec![2u32; 10])?;

        let expected = Matrix::new(vec![7u32; 10])?;
        let actual = m1 + m2;
        assert_eq!(expected, actual);

        let mut m1: Matrix<_, 3, 7> = Matrix::new(vec![9i32; 21])?;
        let m2: Matrix<_, 3, 7> = Matrix::new(vec![1i32; 21])?;

        let expected = Matrix::new(vec![10i32; 21])?;
        m1 += m2;
        assert_eq!(expected, m1);

        Ok(())
    }
}
