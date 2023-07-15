use crate::identity::IdMul;
use std::ops::Div;

pub trait Recip {
    type Output;

    fn recip(self) -> Self::Output;
}

impl<T, U> Recip for T
where
    T: Div<T, Output = U> + IdMul,
{
    type Output = U;

    fn recip(self) -> Self::Output {
        Self::id_mul() / self
    }
}
