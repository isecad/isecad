pub trait Dot {
    type Output;

    fn dot(self, rhs: Self) -> Self::Output;
}
