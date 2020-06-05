pub trait SquareRoot {
    fn square_root(self) -> Self;
}

impl SquareRoot for f32 {
    fn square_root(self) -> Self {
        self.sqrt()
    }
}
