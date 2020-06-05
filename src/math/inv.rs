pub trait Inv {
    fn inv(self) -> Self;
}

impl Inv for f32 {
    fn inv(self) -> Self {
        self.recip()
    }
}
