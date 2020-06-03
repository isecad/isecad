pub trait Magnitude {
    type Output;

    /// Calculates a value proportional to magnitude; e.g., for vectors it may be a squared magnitude.
    ///
    /// A proportional magnitude may be faster than regular magnitude, so it may be useful to compare values.
    fn magnitude_proportional(&self) -> Self::Output;

    /// Calculates a magnitude.
    fn magnitude(&self) -> Self::Output;
}

impl Magnitude for f32 {
    type Output = f32;

    fn magnitude_proportional(&self) -> Self::Output {
        self.abs()
    }

    fn magnitude(&self) -> Self::Output {
        self.abs()
    }
}

impl Magnitude for u8 {
    type Output = u8;

    fn magnitude_proportional(&self) -> Self::Output {
        *self
    }

    fn magnitude(&self) -> Self::Output {
        *self
    }
}
