pub trait Magnitude {
    /// Calculates squared magnitude of the vector; it may be used to compare vectors my magnitude.
    fn magnitude_squared(&self) -> f32;

    /// Calculates magnitude of the vector.
    fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude_squared())
    }
}
