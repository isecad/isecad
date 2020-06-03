pub trait Inv {
    fn inv(&self) -> Self;
}

impl Inv for f32 {
    fn inv(&self) -> Self {
        self.recip()
    }
}

impl Inv for u8 {
    fn inv(&self) -> Self {
        1 / *self
    }
}
