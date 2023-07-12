pub trait Recip {
    type Output;

    fn recip(self) -> Self;
}

impl Recip for f32 {
    type Output = f32;

    fn recip(self) -> Self {
        self.recip()
    }
}

impl Recip for f64 {
    type Output = f64;

    fn recip(self) -> Self {
        self.recip()
    }
}
