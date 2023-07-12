pub trait Inverse {
    fn inverse(self) -> Self;
}

impl Inverse for f32 {
    fn inverse(self) -> Self {
        1.0 / self
    }
}

impl Inverse for f64 {
    fn inverse(self) -> Self {
        1.0 / self
    }
}
