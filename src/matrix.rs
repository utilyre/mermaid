use crate::{
    identity::{IdAdd, IdMul},
    vec2::Vec2,
    vec3::Vec3,
};
use std::{
    array,
    fmt::{self, Display, Formatter},
    ptr,
};

pub mod aliases;
mod iter;
mod ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn new(rows: [[T; N]; M]) -> Self {
        Self(rows)
    }

    pub fn rows(&self) -> [[&T; N]; M] {
        array::from_fn(|i| self.0[i].each_ref())
    }

    pub fn into_rows(self) -> [[T; N]; M] {
        self.0
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

    pub fn map<U, F>(&self, mut f: F) -> Matrix<U, M, N>
    where
        F: FnMut(usize, usize, &T) -> U,
    {
        Matrix::new(array::from_fn(|i| {
            array::from_fn(|j| f(i, j, &self[(i, j)]))
        }))
    }

    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut T),
    {
        for i in 0..M {
            for j in 0..N {
                f(i, j, &mut self[(i, j)]);
            }
        }
    }

    pub fn into_map<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(usize, usize, T) -> T,
    {
        for i in 0..M {
            for j in 0..N {
                let x = f(i, j, unsafe { ptr::read(&self[(i, j)]) });
                unsafe {
                    ptr::write(&mut self[(i, j)], x);
                }
            }
        }

        self
    }
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(array::from_fn(|_| array::from_fn(|_| T::default())))
    }
}

impl<T, const M: usize, const N: usize> IdAdd for Matrix<T, M, N>
where
    T: IdAdd,
{
    fn id_add() -> Self {
        Self::new(array::from_fn(|_| array::from_fn(|_| T::id_add())))
    }
}

impl<T, const M: usize> IdMul for Matrix<T, M, M>
where
    T: IdAdd + IdMul,
{
    fn id_mul() -> Self {
        Self::new(array::from_fn(|i| {
            array::from_fn(|j| if i == j { T::id_mul() } else { T::id_add() })
        }))
    }
}

impl From<Vec2> for Matrix<f32, 2, 1> {
    fn from(value: Vec2) -> Self {
        Matrix::new([[value.x], [value.y]])
    }
}

impl<const N: usize> From<[Vec2; N]> for Matrix<f32, 2, N> {
    fn from(value: [Vec2; N]) -> Self {
        Matrix::new([
            array::from_fn(|i| value[i].x),
            array::from_fn(|i| value[i].y),
        ])
    }
}

impl From<Vec3> for Matrix<f32, 3, 1> {
    fn from(value: Vec3) -> Self {
        Matrix::new([[value.x], [value.y], [value.z]])
    }
}

impl<const N: usize> From<[Vec3; N]> for Matrix<f32, 3, N> {
    fn from(value: [Vec3; N]) -> Self {
        Matrix::new([
            array::from_fn(|i| value[i].x),
            array::from_fn(|i| value[i].y),
            array::from_fn(|i| value[i].z),
        ])
    }
}

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings = self.map(|_, _, x| x.to_string());
        let Some(max_len) = strings
            .0
            .iter()
            .flat_map(|row| row.iter())
            .map(|x| x.len())
            .max()
        else {
            return write!(f, "||");
        };

        for i in 0..M {
            write!(f, "|")?;
            for j in 0..N {
                write!(f, "{: ^len$}", strings[(i, j)], len = max_len + 2)?;
            }
            write!(f, "|")?;

            if i < M - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_mul() {
        let m_3x3 = Matrix::<u32, 3, 3>::id_mul();
        let m_4x4 = Matrix::<u32, 4, 4>::id_mul();

        assert_eq!(Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), m_3x3);
        assert_eq!(
            Matrix::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
            m_4x4
        );
    }

    #[test]
    fn display() {
        let mat = Matrix::new([
            [-1, 0],
            [2, 87],
            [-55, 3],
        ]);

        assert_eq!(
            "\
            | -1    0  |\n\
            |  2   87  |\n\
            | -55   3  |\
            ",
            mat.to_string()
        );
    }
}
