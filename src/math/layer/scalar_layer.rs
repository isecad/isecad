use crate::*;
use std::collections::*;
use std::hash::*;
use std::ops::*;

/// Creates new f32 layer from existing u8 layer.
pub fn from_u8_layer(source: &Layer<u8>) -> Layer<f32> {
    let mut new = Layer::new(source.len());

    for (i, &s_i) in source.iter().enumerate() {
        new[i] = s_i as f32;
    }

    new
}

/// $(\min L, \max L)$
///
/// Returns min and max values of a scalar layer.
pub fn min_max<T>(layer: &Layer<T>) -> (T, T)
where
    T: Copy + PartialOrd + Bounded,
{
    let mut min = T::MAX_BOUND;
    let mut max = T::MIN_BOUND;

    for &s_i in layer.iter() {
        if s_i < min {
            min = s_i;
        }

        if s_i > max {
            max = s_i;
        }
    }

    (min, max)
}

/// Returns index of max element of a scalar layer.
pub fn max_index<T>(layer: &Layer<T>) -> usize
where
    T: Copy + PartialOrd + Bounded,
{
    let mut max = T::MIN_BOUND;
    let mut max_index = 0;

    for (i, &s_i) in layer.iter().enumerate() {
        if s_i > max {
            max = s_i;
            max_index = i;
        }
    }

    max_index
}

/// $O_i = \frac{u_t - l_t}{u_f - l_f} (S_i - l_f) + l_t$
///
/// Rescales a scalar field from specified old range to new range.
///
/// # Arguments
///
/// -   `source` — $S$ — the source layer.
/// -   `output` — $O$ — the output layer to write result into.
/// -   `from_lower` — $l_f$ — lower bound of old range.
/// -   `from_upper` — $u_f$ — upper bound of old range.
/// -   `to_lower` — $l_t$ — lower bound of new range.
/// -   `to_upper` — $u_t$ — upper bound of new range.
pub fn rescale_from_to<T>(source: &Layer<T>, output: &mut Layer<T>, from_lower: T, from_upper: T, to_lower: T, to_upper: T)
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Div<Output = T>,
{
    let scaling_factor = (to_upper - to_lower) / (from_upper - from_lower);

    for (i, &s_i) in source.iter().enumerate() {
        output[i] = (s_i - from_lower) * scaling_factor + to_lower;
    }
}

/// $O_i = \frac{u_t - l_t}{\max S - \min S} (S_i - \min S) + l_t$
///
/// Rescales a scalar field from inferred old range to new range.
///
/// # Arguments
///
/// -   `source` — $S$ — the source layer.
/// -   `output` — $O$ — the output layer to write result into.
/// -   `to_lower` — $l_t$ — lower bound of new range.
/// -   `to_upper` — $u_t$ — upper bound of new range.
pub fn rescale_to<T>(source: &Layer<T>, output: &mut Layer<T>, to_lower: T, to_upper: T)
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Div<Output = T> + PartialOrd + Bounded,
{
    let (from_lower, from_upper) = min_max(source);

    rescale_from_to(source, output, from_lower, from_upper, to_lower, to_upper);
}

/// $O_i = \frac{S_i - \min S}{\max S - \min S}$
///
/// Normalizes a scalar field.
pub fn normalize<T>(source: &Layer<T>, output: &mut Layer<T>)
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd + Bounded + Inv,
{
    let (from_lower, from_upper) = min_max(source);
    let scaling_factor = (from_upper - from_lower).inv();

    for (i, &s_i) in source.iter().enumerate() {
        output[i] = (s_i - from_lower) * scaling_factor;
    }
}

/// Calculates average value of an f32 field.
pub fn average(layer: &Layer<f32>) -> f32 {
    layer.iter().fold(0.0, |sum, &x| sum + x) / layer.len() as f32
}

/// Creates a set of unique values of a layer.
pub fn unique<T>(layer: &Layer<T>) -> HashSet<T>
where
    T: Copy + Hash,
{
    let mut set = HashSet::new();

    for &l_i in layer.iter() {
        set.insert(l_i);
    }

    set
}
