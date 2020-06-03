pub trait ToNumeric<T> {
    fn to_numeric_proportional(&self) -> T;
    fn to_numeric(&self) -> T;
}

impl ToNumeric<Self> for f32 {
    fn to_numeric_proportional(&self) -> Self {
        *self
    }

    fn to_numeric(&self) -> Self {
        *self
    }
}

impl ToNumeric<Self> for u8 {
    fn to_numeric_proportional(&self) -> Self {
        *self
    }

    fn to_numeric(&self) -> Self {
        *self
    }
}
