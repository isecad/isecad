pub trait DivUsize {
    fn div_usize(self, rhs: usize) -> Self;
}

impl DivUsize for f32 {
    fn div_usize(self, rhs: usize) -> Self {
        self / rhs as f32
    }
}
