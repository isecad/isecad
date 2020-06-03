pub trait Similarity {
    type Output;

    fn similarity(self, rhs: Self) -> Self::Output;
}
