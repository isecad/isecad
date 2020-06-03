pub trait Power<T = Self> {
    type Output;

    fn power(self, rhs: T) -> Self::Output;
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
