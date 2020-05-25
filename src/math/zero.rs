pub trait Zero<T> {
    fn zero() -> T;
}

impl Zero<bool> for bool {
    fn zero() -> bool {
        false
    }
}

impl Zero<usize> for usize {
    fn zero() -> usize {
        0
    }
}

impl Zero<isize> for isize {
    fn zero() -> isize {
        0
    }
}

impl Zero<u8> for u8 {
    fn zero() -> u8 {
        0
    }
}

impl Zero<i8> for i8 {
    fn zero() -> i8 {
        0
    }
}

impl Zero<u16> for u16 {
    fn zero() -> u16 {
        0
    }
}

impl Zero<i16> for i16 {
    fn zero() -> i16 {
        0
    }
}

impl Zero<u32> for u32 {
    fn zero() -> u32 {
        0
    }
}

impl Zero<i32> for i32 {
    fn zero() -> i32 {
        0
    }
}

impl Zero<u64> for u64 {
    fn zero() -> u64 {
        0
    }
}

impl Zero<i64> for i64 {
    fn zero() -> i64 {
        0
    }
}

impl Zero<f32> for f32 {
    fn zero() -> f32 {
        0.0
    }
}

impl Zero<f64> for f64 {
    fn zero() -> f64 {
        0.0
    }
}
