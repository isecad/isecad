pub trait Bounded<T> {
    const MIN_BOUND: T;
    const MAX_BOUND: T;
}

impl Bounded<bool> for bool {
    const MIN_BOUND: bool = false;
    const MAX_BOUND: bool = true;
}

impl Bounded<usize> for usize {
    const MIN_BOUND: usize = usize::MIN;
    const MAX_BOUND: usize = usize::MAX;
}

impl Bounded<isize> for isize {
    const MIN_BOUND: isize = isize::MIN;
    const MAX_BOUND: isize = isize::MAX;
}

impl Bounded<u8> for u8 {
    const MIN_BOUND: u8 = u8::MIN;
    const MAX_BOUND: u8 = u8::MAX;
}

impl Bounded<i8> for i8 {
    const MIN_BOUND: i8 = i8::MIN;
    const MAX_BOUND: i8 = i8::MAX;
}

impl Bounded<u16> for u16 {
    const MIN_BOUND: u16 = u16::MIN;
    const MAX_BOUND: u16 = u16::MAX;
}

impl Bounded<i16> for i16 {
    const MIN_BOUND: i16 = i16::MIN;
    const MAX_BOUND: i16 = i16::MAX;
}

impl Bounded<u32> for u32 {
    const MIN_BOUND: u32 = u32::MIN;
    const MAX_BOUND: u32 = u32::MAX;
}

impl Bounded<i32> for i32 {
    const MIN_BOUND: i32 = i32::MIN;
    const MAX_BOUND: i32 = i32::MAX;
}

impl Bounded<u64> for u64 {
    const MIN_BOUND: u64 = u64::MIN;
    const MAX_BOUND: u64 = u64::MAX;
}

impl Bounded<i64> for i64 {
    const MIN_BOUND: i64 = i64::MIN;
    const MAX_BOUND: i64 = i64::MAX;
}

impl Bounded<f32> for f32 {
    const MIN_BOUND: f32 = f32::NEG_INFINITY;
    const MAX_BOUND: f32 = f32::INFINITY;
}

impl Bounded<f64> for f64 {
    const MIN_BOUND: f64 = f64::NEG_INFINITY;
    const MAX_BOUND: f64 = f64::INFINITY;
}
