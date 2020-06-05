pub trait Power<Rhs = Self> {
    type Output;

    fn power(self, rhs: Rhs) -> Self::Output;
}

impl Power for f32 {
    type Output = Self;

    fn power(self, rhs: Self) -> Self::Output {
        self.powf(rhs)
    }
}

impl Power<i32> for f32 {
    type Output = Self;

    fn power(self, rhs: i32) -> Self::Output {
        self.powi(rhs)
    }
}
