pub trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}
