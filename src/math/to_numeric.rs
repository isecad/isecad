pub trait ToNumeric<T> {
    fn into_numeric_proportional(self) -> T;
    fn into_numeric(self) -> T;
}

impl ToNumeric<Self> for f32 {
    fn into_numeric_proportional(self) -> Self {
        self
    }

    fn into_numeric(self) -> Self {
        self
    }
}

impl ToNumeric<Self> for u8 {
    fn into_numeric_proportional(self) -> Self {
        self
    }

    fn into_numeric(self) -> Self {
        self
    }
}
