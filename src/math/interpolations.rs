use crate::*;
use std::ops::*;

pub fn mix<T>(x: T, a: T, b: T) -> T
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    x * (b - a) + a
}

pub fn clamp<T>(x: T, min: T, max: T) -> T
where
    T: Copy + PartialOrd,
{
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn step<T>(x: T, a: T) -> T
where
    T: Default + PartialOrd + One,
{
    if x > a {
        T::one()
    } else {
        T::default()
    }
}

pub fn linearstep<T>(x: T, a: T, b: T) -> T
where
    T: Copy + Default + PartialOrd + Mul<Output = T> + Sub<Output = T> + Inv + One,
{
    clamp((x - a) * (b - a).inv(), T::default(), T::one())
}

pub fn smoothstep<T>(x: T, a: T, b: T) -> T
where
    T: Copy + Default + PartialOrd + Mul<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Add<f32, Output = T> + Inv + One,
{
    let l = linearstep(x, a, b);

    l * l * (l * -2.0 + 3.0)
}

pub fn linearstep_inv<T>(x: T, a: T, inv_diff: T) -> T
where
    T: Copy + Default + PartialOrd + Mul<Output = T> + Sub<Output = T> + One,
{
    clamp((x - a) * inv_diff, T::default(), T::one())
}

pub fn smoothstep_inv<T>(x: T, a: T, inv_diff: T) -> T
where
    T: Copy + Default + PartialOrd + Mul<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Add<f32, Output = T> + One,
{
    let l = linearstep_inv(x, a, inv_diff);

    l * l * (l * -2.0 + 3.0)
}

pub fn smoothstep2<T>(x: T, k: T) -> T
where
    T: Copy + Neg<Output = T> + Mul<Output = T> + Mul<f32, Output = T> + Add<f32, Output = T> + EX<Output = T> + Inv,
{
    ((-k * x).e_x() + 1.0).inv() * 2.0 + -1.0
}

pub fn smoothstep2_neg<T>(x: T, k: T) -> T
where
    T: Copy + Mul<Output = T> + Mul<f32, Output = T> + Add<f32, Output = T> + EX<Output = T> + Inv,
{
    ((k * x).e_x() + 1.0).inv() * 2.0 + -1.0
}

pub fn lerp<T>(x: T, xs: &[T], ys: &[T]) -> T
where
    T: Copy + Default + PartialOrd + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Inv + One,
{
    let mut result = ys[0];

    for (i, &x_i) in xs[1..].iter().enumerate() {
        result = mix(linearstep(x, xs[i - 1], x_i), ys[i], result);
    }

    result
}
