pub trait Normalize {
    /// Normalizes a value; for vectors it will transform a vector to a unit vector.
    fn normalize(self) -> Self;
}
