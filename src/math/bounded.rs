pub trait Bounded {
    fn min_bound<'a>() -> &'a Self;
    fn max_bound<'a>() -> &'a Self;
}

impl Bounded for bool {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &false
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &true
    }
}

impl Bounded for usize {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &usize::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &usize::MAX
    }
}

impl Bounded for isize {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &isize::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &isize::MAX
    }
}

impl Bounded for u8 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &u8::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &u8::MAX
    }
}

impl Bounded for i8 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &i8::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &i8::MAX
    }
}

impl Bounded for u16 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &u16::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &u16::MAX
    }
}

impl Bounded for i16 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &i16::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &i16::MAX
    }
}

impl Bounded for u32 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &u32::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &u32::MAX
    }
}

impl Bounded for i32 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &i32::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &i32::MAX
    }
}

impl Bounded for u64 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &u64::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &u64::MAX
    }
}

impl Bounded for i64 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &i64::MIN
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &i64::MAX
    }
}

impl Bounded for f32 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &f32::NEG_INFINITY
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &f32::INFINITY
    }
}

impl Bounded for f64 {
    #[inline]
    fn min_bound<'a>() -> &'a Self {
        &f64::NEG_INFINITY
    }

    #[inline]
    fn max_bound<'a>() -> &'a Self {
        &f64::INFINITY
    }
}
