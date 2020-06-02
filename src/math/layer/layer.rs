#![allow(dead_code)]

use crate::*;

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

impl From<Layer<u8>> for Layer<f32> {
    fn from(source: Layer<u8>) -> Layer<f32> {
        let mut new = Layer::new(source.len());

        for (i, &s_i) in source.iter().enumerate() {
            new[i] = s_i as f32;
        }

        new
    }
}

impl<T: Default + Copy> Layer<T> {
    // region Core functionality
    /// Creates new layer of specified length.
    pub fn new(length: usize) -> Self {
        let mut vec = Vec::with_capacity(length);

        vec.resize_with(length, Default::default);

        Self(vec.into_boxed_slice())
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
        for (i, &m_i) in mapping.iter().enumerate() {
            output[m_i] = self[m_i] + add[i];
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
    /// Returns min and max values of layer at once.
    pub fn min_max(&self) -> (T, T)
    where
        T: PartialOrd + Bounded,
    {
        let mut min = Bounded::MAX_BOUND;
        let mut max = Bounded::MIN_BOUND;

        for &s_i in self.iter() {
            if s_i < min {
                min = s_i;
            }

            if s_i > max {
                max = s_i;
            }
        }

        (min, max)
    }

    /// Returns indices of elements with min and max values.
    pub fn min_max_indices(&self) -> (usize, usize)
    where
        T: PartialOrd + Bounded,
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
