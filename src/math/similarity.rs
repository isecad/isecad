pub trait Similarity<Rhs = Self> {
    type Output;

    fn similarity(self, rhs: Rhs) -> Self::Output;
}
