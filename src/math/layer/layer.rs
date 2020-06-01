#![allow(dead_code)]

use crate::*;
use std::cmp::*;

pub trait Layer<T> {
    // region Core functionality
    /// Creates new layer of specified length.
    fn of_length(length: usize) -> Box<Self>
    where
        T: Default;

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
    fn swizzle(&self, mapping: &[usize], output: &mut Self)
    where
        T: Copy;

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
    fn inverse_swizzle_add(&self, mapping: &[usize], add: &Self, output: &mut Self)
    where
        T: Copy + std::ops::Add<Output = T>;
    // endregion Core functionality

    // region Statistics
    /// Returns min and max values of layer at once.
    fn min_max(&self) -> (T, T)
    where
        T: Copy + PartialOrd + Bounded;

    /// Returns indices of elements with min and max values.
    fn min_max_indices(&self) -> (usize, usize)
    where
        T: Copy + PartialOrd + Bounded;
    // endregion Statistics
}

impl<T> Layer<T> for [T] {
    // region Core functionality
    fn of_length(length: usize) -> Box<Self>
    where
        T: Default,
    {
        let mut vec = Vec::<T>::with_capacity(length);

        vec.resize_with(length, Default::default);

        vec.into_boxed_slice()
    }

    fn swizzle(&self, mapping: &[usize], output: &mut Self)
    where
        T: Copy,
    {
        for (i, &m_i) in mapping.iter().enumerate() {
            output[i] = self[m_i];
        }
    }

    fn inverse_swizzle_add(&self, mapping: &[usize], add: &Self, output: &mut Self)
    where
        T: Copy + std::ops::Add<Output = T>,
    {
        for (i, &m_i) in mapping.iter().enumerate() {
            output[m_i] = self[m_i] + add[i];
        }
    }
    // endregion Core functionality

    // region Statistics
    fn min_max(&self) -> (T, T)
    where
        T: Copy + PartialOrd + Bounded,
    {
        let mut min = Bounded::MAX_BOUND;
        let mut max = Bounded::MIN_BOUND;

        for &s_i in self {
            if s_i < min {
                min = s_i;
            }

            if s_i > max {
                max = s_i;
            }
        }

        (min, max)
    }

    fn min_max_indices(&self) -> (usize, usize)
    where
        T: Copy + PartialOrd + Bounded,
    {
        let mut min = Bounded::MAX_BOUND;
        let mut max = Bounded::MIN_BOUND;

        let mut max_index = 0;
        let mut min_index = 0;

        for (i, &s_i) in self.iter().enumerate() {
            if s_i < min {
                min = s_i;
                min_index = i;
            }

            if s_i > max {
                max = s_i;
                max_index = i;
            }
        }

        (min_index, max_index)
    }
    // endregion Statistics
}
