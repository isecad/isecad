use crate::*;
use std::ops::*;

/// $\max |L|$, where $|L|$ is a field of magnitudes of $L$.
///
/// Returns max magnitude of a vector layer.
pub fn max<T>(layer: &Layer<T>) -> f32
where
    T: Magnitude,
{
    let mut max = 0.0;

    for s_i in layer.iter() {
        let mag_sqr = s_i.magnitude_squared();

        if mag_sqr > max {
            max = mag_sqr;
        }
    }

    f32::sqrt(max)
}

/// $O_i = \frac{m_t}{m_f} S_i$
///
/// Rescales a vector field from specified old magnitude to new magnitude.
///
/// # Arguments
///
/// -   `source` — $S$ — the source vector field.
/// -   `output` — $O$ — the output vector field to write result into.
/// -   `from_magnitude` — $m_f$ — the magnitude to rescale vectors from.
/// -   `to_magnitude` — $m_t$ — the magnitude to rescale vectors to.
pub fn rescale_from_to<T>(source: &Layer<T>, output: &mut Layer<T>, from_magnitude: f32, to_magnitude: f32)
where
    T: Copy + Mul<f32, Output = T>,
{
    let scaling_factor = to_magnitude / from_magnitude;

    for (i, &s_i) in source.iter().enumerate() {
        output[i] = s_i * scaling_factor;
    }
}

/// $O_i = \frac{m_t}{\max |S|} S_i$, where $|S|$ is a field of magnitudes of $S$.
///
/// Rescales a vector field from inferred current magnitude to new magnitude.
///
/// # Arguments
///
/// -   `source` — $S$ — the source vector field.
/// -   `output` — $O$ — the output vector field to write result into.
/// -   `to_magnitude` — $m_t$ — the magnitude to rescale vectors to.
pub fn rescale_to<T>(source: &Layer<T>, output: &mut Layer<T>, to_magnitude: f32)
where
    T: Copy + Mul<f32, Output = T> + Magnitude,
{
    rescale_from_to(source, output, max(source), to_magnitude);
}

/// Normalizes each vector in a field.
pub fn normalize_each<T>(source: &Layer<T>, output: &mut Layer<T>)
where
    T: Normalize,
{
    for (i, s_i) in source.iter().enumerate() {
        output[i] = s_i.normalize()
    }
}

pub fn weighted_average<T>(layer: &Layer<T>, weights: &Layer<f32>) -> T
where
    T: Copy + Default + Add<Output = T> + Mul<f32, Output = T> + Div<f32, Output = T>,
{
    let (sum, weights_sum) = layer
        .iter()
        .zip(weights.iter())
        .fold((T::default(), 0.0), |(sum, weights_sum), (&l_i, &w_i)| (sum + l_i * w_i, weights_sum + w_i));

    sum / weights_sum
}
