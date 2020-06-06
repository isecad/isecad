pub trait EX {
    type Output;

    fn e_x(self) -> Self::Output;
}

impl EX for f32 {
    type Output = Self;

    fn e_x(self) -> Self::Output {
        self.exp()
    }
}
