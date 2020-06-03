#![allow(dead_code)]

use crate::*;
use std::cmp::*;
use std::collections::*;
use std::hash::*;
use std::ops::*;

pub struct Layer<T>(Box<[T]>);

impl<T> std::ops::Deref for Layer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T> std::ops::DerefMut for Layer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl<'a, T: Default + Copy> IntoIterator for &'a Layer<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Default + Copy> Layer<T> {
    // region Core functionality
    /// Creates new layer of specified length.
    pub fn new(length: usize) -> Self {
        let mut vec = Vec::with_capacity(length);

        vec.resize_with(length, T::default);

        Self(vec.into_boxed_slice())
    }

    /// Converts a layer into another type.
    pub fn convert<U>(&self) -> Layer<U>
    where
        T: Into<U>,
        U: Copy + Default,
    {
        let mut new = Layer::new(self.len());

        for (i, &s_i) in self.iter().enumerate() {
            new[i] = s_i.into();
        }

        new
    }

    /// $O_i = S_{M_i}$
    ///
    /// | $S$                 | $M$                             | $O$             |
    /// | :-----------------: | :-----------------------------: | :-------------: |
    /// | $S_0 = 8$           | $M_0 = 4$ — from $S_4$ to $O_0$ | $O_0 = S_4 = 1$ |
    /// | $S_1 = \varnothing$ | $M_1 = 2$ — from $S_2$ to $O_1$ | $O_1 = S_2 = 4$ |
    /// | $S_2 = 4$           | $M_2 = 0$ — from $S_0$ to $O_2$ | $O_2 = S_0 = 8$ |
    /// | $S_3 = 8$           | $M_3 = 3$ — from $S_3$ to $O_3$ | $O_3 = S_3 = 8$ |
    /// | $S_4 = 1$           |                                 |                 |
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the source layer to copy items from.
    /// -   `mapping` — $M$ — the mapping layer.
    /// -   `output` — $O$ — the output layer to copy items into.
    pub fn swizzle(&self, mapping: &[usize], output: &mut Self) {
        for (i, &m_i) in mapping.iter().enumerate() {
            output[i] = self[m_i];
        }
    }

    /// $O_{M_i} = S_{M_i} + A_i$
    ///
    /// | $S$    | $M$    | $A$    | $O$         |
    /// | :----: | :----: | :----: | :--------:  |
    /// | $1$    | $1$    | $2$    | $1$         |
    /// | $2$    | $4$    | $-9$   | $4 = 2 + 2$ |
    /// | $3$    | $3$    | $8$    | $8 = 3 + 5$ |
    /// | $0$    | $2$    | $5$    | $8 = 0 + 8$ |
    /// | $9$    |        |        | $0 = 9 - 9$ |
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the source layer.
    /// -   `mapping` — $M$ — the mapping layer.
    /// -   `add` — $A$ — the layer of values to add to the $S$ values.
    /// -   `output` — $O$ — the output layer to write result into.
    pub fn inverse_swizzle_add(&self, mapping: &[usize], add: &Self, output: &mut Self)
    where
        T: std::ops::Add<Output = T>,
    {
        for (&m_i, &a_i) in mapping.iter().zip(add) {
            output[m_i] = self[m_i] + a_i;
        }
    }

    /// Fills a layer with specified value.
    ///
    /// Current implementation uses the `[T]::fill`, but in the future versions we will use WASM `memory.{fill,copy}` when possible instead.
    pub fn fill(&mut self, value: T) {
        self.as_mut().fill(value);
    }

    /// Copies values from this layer to the output.
    ///
    /// Current implementation uses the `[T]::copy_from_slice`, but in the future versions we will use WASM `memory.copy` instead.
    pub fn copy_into(&self, output: &mut Self) {
        output.copy_from_slice(self);
    }
    // endregion Core functionality

    // region Statistics
    /// Calculates average value of an field.
    pub fn average(&self) -> T
    where
        T: Add<Output = T> + DivUsize,
    {
        self.iter().fold(T::default(), |sum, &x| sum + x).div_usize(self.len())
    }

    /// Calculates weighted average of a field using given weights layer.
    pub fn weighted_average<U>(&self, weights: &Layer<U>) -> T
    where
        T: Add<Output = T> + Mul<U, Output = T> + Div<U, Output = T>,
        U: Copy + Default + Add<Output = U>,
    {
        let (sum, weights_sum) = self.iter().zip(weights).fold((T::default(), U::default()), |(sum, weights_sum), (&l_i, &w_i)| {
            (sum + l_i * w_i, weights_sum + w_i)
        });

        sum / weights_sum
    }

    /// Creates a set of unique values of a layer.
    pub fn unique(&self) -> HashSet<T>
    where
        T: Eq + Hash,
    {
        let mut set = HashSet::new();

        for &l_i in self {
            set.insert(l_i);
        }

        set
    }

    /// $(\min S, \max S)$
    pub fn min_max<U>(&self) -> (T, T)
    where
        T: ToNumeric<U>,
        U: Copy + PartialOrd + Bounded,
    {
        let mut min_proportional = U::MAX_BOUND;
        let mut max_proportional = U::MIN_BOUND;

        let mut min = T::default();
        let mut max = T::default();

        for &s_i in self {
            let numeric_proportional = s_i.to_numeric_proportional();

            if numeric_proportional < min_proportional {
                min_proportional = numeric_proportional;
                min = s_i;
            }

            if numeric_proportional > max_proportional {
                max_proportional = numeric_proportional;
                max = s_i;
            }
        }

        (min, max)
    }

    /// Returns index of max element of a scalar layer.
    pub fn scalar_max_index(&self) -> usize
    where
        T: PartialOrd + Bounded,
    {
        let mut max = T::MIN_BOUND;
        let mut max_index = 0;

        for (i, &s_i) in self.iter().enumerate() {
            if s_i > max {
                max = s_i;
                max_index = i;
            }
        }

        max_index
    }

    /// $O_i = \frac{u_t - l_t}{u_f - l_f} (S_i - l_f) + l_t$
    ///
    /// Rescales a field from specified old range to new range.
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the source layer.
    /// -   `from_lower` — $l_f$ — lower bound of old range.
    /// -   `from_upper` — $u_f$ — upper bound of old range.
    /// -   `to_lower` — $l_t$ — lower bound of new range.
    /// -   `to_upper` — $u_t$ — upper bound of new range.
    /// -   `output` — $O$ — the output layer to write result into.
    pub fn rescale_from_to_range<U>(&self, from_lower: U, from_upper: U, to_lower: U, to_upper: U, output: &mut Self)
    where
        T: Sub<U, Output = T> + Mul<U, Output = T> + Add<U, Output = T>,
        U: Copy + Sub<Output = U> + Div<Output = U>,
    {
        let scaling_factor = (to_upper - to_lower) / (from_upper - from_lower);

        for (i, &s_i) in self.iter().enumerate() {
            output[i] = (s_i - from_lower) * scaling_factor + to_lower;
        }
    }

    /// $O_i = \frac{u_t - l_t}{\max S - \min S} (S_i - \min S) + l_t$
    ///
    /// Rescales a field from inferred old range to new range.
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the source layer.
    /// -   `to_lower` — $l_t$ — lower bound of new range.
    /// -   `to_upper` — $u_t$ — upper bound of new range.
    /// -   `output` — $O$ — the output layer to write result into.
    pub fn rescale_to_range<U>(&self, to_lower: U, to_upper: U, output: &mut Layer<T>)
    where
        T: Sub<U, Output = T> + Mul<U, Output = T> + Add<U, Output = T> + ToNumeric<U>,
        U: Copy + Sub<Output = U> + Div<Output = U> + PartialOrd + Bounded,
    {
        let (from_lower, from_upper) = self.min_max();

        let from_lower = from_lower.to_numeric();
        let from_upper = from_upper.to_numeric();

        self.rescale_from_to_range(from_lower, from_upper, to_lower, to_upper, output);
    }

    /// $O_i = \frac{S_i - \min S}{\max S - \min S}$
    ///
    /// Rescales a field to the $[0, 1]$ range.
    pub fn normalize<U>(&self, output: &mut Self)
    where
        T: Sub<U, Output = T> + Mul<U, Output = T> + ToNumeric<U>,
        U: Copy + Sub<Output = U> + PartialOrd + Bounded + Inv,
    {
        let (from_lower, from_upper) = self.min_max();

        let from_lower = from_lower.to_numeric();
        let from_upper = from_upper.to_numeric();

        let scaling_factor = (from_upper - from_lower).inv();

        for (i, &s_i) in self.iter().enumerate() {
            output[i] = (s_i - from_lower) * scaling_factor;
        }
    }

    /// $O_i = \frac{u_t}{u_f} S_i$
    ///
    /// Rescales a field from the $[0, u_f]$ range to the $[0, u_t]$ range.
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the source layer.
    /// -   `from_upper` — $u_f$ — upper bound of old range.
    /// -   `to_upper` — $u_t$ — upper bound of new range.
    /// -   `output` — $O$ — the output layer to write result into.
    pub fn rescale_from_to<U>(&self, from_upper: U, to_upper: U, output: &mut Self)
    where
        T: Mul<U, Output = T>,
        U: Copy + Div<Output = U>,
    {
        let scaling_factor = to_upper / from_upper;

        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i * scaling_factor;
        }
    }

    /// $O_i = \frac{u_t}{\max S} S_i$
    ///
    /// Rescales a scalar field from inferred $[0, \max S]$ range to the $[0, u_t]$ range.
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the source layer.
    /// -   `to_upper` — $u_t$ — upper bound of new range.
    /// -   `output` — $O$ — the output layer to write result into.
    pub fn rescale_to<U>(&self, to_upper: U, output: &mut Layer<T>)
    where
        T: Mul<U, Output = T> + ToNumeric<U>,
        U: Copy + Div<Output = U> + PartialOrd + Bounded,
    {
        let (_, from_upper) = self.min_max();

        let from_upper = from_upper.to_numeric();

        self.rescale_from_to(from_upper, to_upper, output);
    }

    /// Normalizes each value in a field.
    pub fn normalize_each(&self, output: &mut Self)
    where
        T: Normalize,
    {
        for (i, s_i) in self.iter().enumerate() {
            output[i] = s_i.normalize();
        }
    }
    // endregion Statistics

    // region Field operations
    // region Layer ⋅ Layer
    /// $O_i = \min(S_i, B_i)$
    pub fn min_layer(self, layer_b: &Self, output: &mut Self)
    where
        T: PartialOrd,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = if s_i < b_i { s_i } else { b_i };
        }
    }

    /// $O_i = \max(S_i, B_i)$
    pub fn max_layer(&self, layer_b: &Self, output: &mut Self)
    where
        T: PartialOrd,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = if s_i <= b_i { b_i } else { s_i };
        }
    }

    /// $O_i = S_i < B_i$
    pub fn get_lt_layer_mask(&self, layer_b: &Self, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i < b_i;
        }
    }

    /// $O_i = S_i > B_i$
    pub fn get_gt_layer_mask(&self, layer_b: &Self, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i > b_i;
        }
    }

    /// $O_i = S_i \le B_i$
    pub fn get_le_layer_mask(&self, layer_b: &Self, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i <= b_i;
        }
    }

    /// $O_i = S_i \ge B_i$
    pub fn get_ge_layer_mask(&self, layer_b: &Self, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i >= b_i;
        }
    }

    /// $O_i = S_i = B_i$
    pub fn get_eq_layer_mask(&self, layer_b: &Self, output: &mut Layer<bool>)
    where
        T: PartialEq,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i == b_i;
        }
    }

    /// $O_i = S_i \ne B_i$
    pub fn get_ne_layer_mask(&self, layer_b: &Self, output: &mut Layer<bool>)
    where
        T: PartialEq,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i != b_i;
        }
    }

    /// $O_i = \begin{cases}
    ///     S_i + B_i, & \text{if}   & M_{i \mod L_M} = \texttt{true} \\
    ///     S_i,       & \text{else} &                                \\
    /// \end{cases}$
    pub fn add_layer_by_mask<U>(&self, layer_b: &Layer<U>, mask: &Layer<bool>, output: &mut Self)
    where
        T: Add<U, Output = T>,
        U: Copy + Default,
    {
        let l_m = mask.len();

        for ((i, &a_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = if mask[i % l_m] { a_i + b_i } else { a_i };
        }
    }

    /// $O_i = \begin{cases}
    ///     S_i - B_i, & \text{if}   & M_{i \mod L_M} = \texttt{true} \\
    ///     S_i,       & \text{else} &                                \\
    /// \end{cases}$
    pub fn sub_layer_by_mask<U>(&self, layer_b: &Layer<U>, mask: &Layer<bool>, output: &mut Self)
    where
        T: Sub<U, Output = T>,
        U: Copy + Default,
    {
        let l_m = mask.len();

        for ((i, &a_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = if mask[i % l_m] { a_i - b_i } else { a_i };
        }
    }

    /// $O_i = S_i + W_{i \mod L_W} B_i$
    pub fn add_layer_weighted<U, W, X>(&self, layer_b: &Layer<U>, weights: &Layer<W>, output: &mut Self)
    where
        T: Add<X, Output = T>,
        U: Copy + Default,
        W: Copy + Default + Mul<U, Output = X>,
    {
        let l_w = weights.len();

        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i + weights[i % l_w] * b_i;
        }
    }

    /// $O_i = S_i - W_{i \mod L_W} B_i$
    pub fn sub_layer_weighted<U, W, X>(&self, layer_b: &Layer<U>, weights: &Layer<W>, output: &mut Self)
    where
        T: Sub<X, Output = T>,
        U: Copy + Default,
        W: Copy + Default + Mul<U, Output = X>,
    {
        let l_w = weights.len();

        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i - weights[i % l_w] * b_i;
        }
    }

    /// $O_i = S_i + B_i$
    pub fn add_layer<U>(&self, layer_b: &Layer<U>, output: &mut Self)
    where
        T: Add<U, Output = T>,
        U: Copy + Default,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i + b_i;
        }
    }

    /// $O_i = S_i - B_i$
    pub fn sub_layer<U>(&self, layer_b: &Layer<U>, output: &mut Self)
    where
        T: Sub<U, Output = T>,
        U: Copy + Default,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i - b_i;
        }
    }

    /// $O_i = S_i B_i$
    pub fn mul_layer<U>(&self, layer_b: &Layer<U>, output: &mut Self)
    where
        T: Mul<U, Output = T>,
        U: Copy + Default,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i * b_i;
        }
    }

    /// $O_i = \frac{S_i}{B_i}$
    pub fn div_layer<U>(&self, layer_b: &Layer<U>, output: &mut Self)
    where
        T: Div<U, Output = T>,
        U: Copy + Default,
    {
        for ((i, &s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i / b_i;
        }
    }

    // $O_i = S_i^{B_i}$
    pub fn power_layer<U>(&self, layer_b: &Layer<U>, output: &mut Self)
    where
        T: Power<U, Output = T>,
        U: Copy + Default,
    {
        for ((i, s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i.power(b_i);
        }
    }

    /// $O_i = S_i \cdot B_i$
    pub fn dot_layer<U>(&self, layer_b: &Self, output: &mut Layer<U>)
    where
        T: Dot<Output = U>,
        U: Copy + Default,
    {
        for ((i, s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i.dot(b_i);
        }
    }

    /// $O_i = \frac{S_i \cdot B_i}{\sqrt{|S_i||B_i|}}$
    pub fn similarity_layer<U>(&self, layer_b: &Self, output: &mut Layer<U>)
    where
        T: Similarity<Output = U>,
        U: Copy + Default,
    {
        for ((i, s_i), &b_i) in self.iter().enumerate().zip(layer_b) {
            output[i] = s_i.similarity(b_i);
        }
    }
    // endregion Layer ⋅ Layer

    // region Layer ⋅ Value
    /// $O_i = \min(S_i, s)$
    pub fn cut_top(&self, value: T, output: &mut Self)
    where
        T: PartialOrd,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = if s_i < value { s_i } else { value };
        }
    }

    /// $O_i = \max(S_i, s)$
    pub fn cut_bottom(&self, value: T, output: &mut Self)
    where
        T: PartialOrd,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = if s_i > value { s_i } else { value };
        }
    }

    /// $O_i = S_i < v$
    pub fn get_lt_value_mask(&self, value: T, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i < value;
        }
    }

    /// $O_i = S_i > v$
    pub fn get_gt_value_mask(&self, value: T, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i > value;
        }
    }

    /// $O_i = S_i \le v$
    pub fn get_le_value_mask(&self, value: T, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i <= value;
        }
    }

    /// $O_i = S_i \ge v$
    pub fn get_ge_value_mask(&self, value: T, output: &mut Layer<bool>)
    where
        T: PartialOrd,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i >= value;
        }
    }

    /// $O_i = S_i = v$
    pub fn get_eq_value_mask(&self, value: T, output: &mut Layer<bool>)
    where
        T: PartialEq,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i == value;
        }
    }

    /// $O_i = S_i \ne v$
    pub fn get_ne_value_mask(&self, value: T, output: &mut Layer<bool>)
    where
        T: PartialEq,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i != value;
        }
    }

    /// $O_i = \begin{cases}
    ///     S_i + v, & \text{if}   & M_{i \mod L_M} = \texttt{true} \\
    ///     S_i,     & \text{else} &                                \\
    /// \end{cases}$
    pub fn add_value_by_mask<U>(&self, value: U, mask: &Layer<bool>, output: &mut Self)
    where
        T: Add<U, Output = T>,
        U: Copy,
    {
        let l_m = mask.len();

        for (i, &a_i) in self.iter().enumerate() {
            output[i] = if mask[i % l_m] { a_i + value } else { a_i };
        }
    }

    /// $O_i = \begin{cases}
    ///     S_i - v, & \text{if}   & M_{i \mod L_M} = \texttt{true} \\
    ///     S_i,     & \text{else} &                                \\
    /// \end{cases}$
    pub fn sub_value_by_mask<U>(&self, value: U, mask: &Layer<bool>, output: &mut Self)
    where
        T: Sub<U, Output = T>,
        U: Copy,
    {
        let l_m = mask.len();

        for (i, &a_i) in self.iter().enumerate() {
            output[i] = if mask[i % l_m] { a_i - value } else { a_i };
        }
    }

    /// $O_i = S_i + W_{i \mod L_W} v$
    pub fn add_value_weighted<U, W, X>(&self, value: U, weights: &Layer<W>, output: &mut Self)
    where
        T: Add<X, Output = T>,
        U: Copy,
        W: Copy + Default + Mul<U, Output = X>,
    {
        let l_w = weights.len();

        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i + weights[i % l_w] * value;
        }
    }

    /// $O_i = S_i - W_{i \mod L_W} v$
    pub fn sub_value_weighted<U, W, X>(&self, weights: &Layer<W>, value: U, output: &mut Self)
    where
        T: Sub<X, Output = T>,
        W: Copy + Default + Mul<U, Output = X>,
        U: Copy,
    {
        let l_w = weights.len();

        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i - weights[i % l_w] * value;
        }
    }

    // $O_i = v S_i$
    pub fn mul_value<U>(&self, value: U, output: &mut Self)
    where
        T: Mul<U, Output = T>,
        U: Copy,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i * value;
        }
    }

    // $O_i = \frac{S_i}{v}$
    pub fn div_value<U>(&self, value: U, output: &mut Self)
    where
        T: Div<U, Output = T>,
        U: Copy,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i / value;
        }
    }

    // $O_i = S_i + v$
    pub fn add_value<U>(&self, value: U, output: &mut Self)
    where
        T: Add<U, Output = T>,
        U: Copy,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i + value;
        }
    }

    // $O_i = S_i - v$
    pub fn sub_value<U>(&self, value: U, output: &mut Self)
    where
        T: Sub<U, Output = T>,
        U: Copy,
    {
        for (i, &s_i) in self.iter().enumerate() {
            output[i] = s_i - value;
        }
    }

    // $O_i = S_i^v$
    pub fn power_value<U>(&self, value: U, output: &mut Self)
    where
        T: Power<U, Output = T>,
        U: Copy,
    {
        for (i, s_i) in self.iter().enumerate() {
            output[i] = s_i.power(value);
        }
    }

    /// $O_i = S_i \cdot v$
    pub fn dot_value<U>(&self, value: T, output: &mut Layer<U>)
    where
        T: Dot<Output = U>,
        U: Copy + Default,
    {
        for (i, s_i) in self.iter().enumerate() {
            output[i] = s_i.dot(value);
        }
    }

    /// $O_i = \frac{S_i \cdot v}{\sqrt{|S_i||v|}}$
    pub fn similarity_value<U>(&self, value: T, output: &mut Layer<U>)
    where
        T: Similarity<Output = U>,
        U: Copy + Default,
    {
        for (i, s_i) in self.iter().enumerate() {
            output[i] = s_i.similarity(value);
        }
    }
    // endregion Layer ⋅ Value

    // region Misc
    /// $O_i = S_i^{-1}$
    pub fn inv(&self, output: &mut Self)
    where
        T: Inv,
    {
        for (i, s_i) in self.iter().enumerate() {
            output[i] = s_i.inv();
        }
    }

    /// $O_i = |S_i|$
    pub fn to_magnitudes<U>(&self, output: &mut Layer<U>)
    where
        T: Magnitude<Output = U>,
        U: Copy + Default,
    {
        for (i, s_i) in self.iter().enumerate() {
            output[i] = s_i.magnitude();
        }
    }
    // endregion Misc
    // endregion Field operations
}
