pub trait Bounded {
    const MIN_BOUND: Self;
    const MAX_BOUND: Self;
}

impl Bounded for f32 {
    const MIN_BOUND: Self = f32::NEG_INFINITY;
    const MAX_BOUND: Self = f32::INFINITY;
}
