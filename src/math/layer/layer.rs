#![allow(dead_code)]

use crate::*;
use std::cmp::*;

pub trait Layer<T> {
    fn min_max(&self) -> (T, T);

    /// For each $M_i$ in the $M$, it takes $F_{M_i}$, and copies it to $S_i$.
    ///
    /// Each $M_i \in [0, L_F)$, where $L_F$ is the length of the $F$.
    ///
    /// $L_M <= L_S, L_M <= L_F$, where $L_M$ it the length of the $M$, $L_S$ is the length of the $S$.
    ///
    /// | $F$           | $M$                       | $S$    |
    /// | :-----------: | :-----------------------: | :----: |
    /// | $8$           | $4$ — from $F_4$ to $S_0$ | $1$    |
    /// | $\varnothing$ | $2$ — from $F_2$ to $S_1$ | $4$    |
    /// | $4$           | $0$ — from $F_0$ to $S_2$ | $8$    |
    /// | $8$           | $3$ — from $F_3$ to $S_3$ | $8$    |
    /// | $1$           |                           |        |
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the target layer.
    /// -   `mapping` — $M$ — the mapping layer.
    /// -   `from` — $F$ — the source layer to copy items from.
    fn swizzle(&mut self, mapping: &[usize], from: &Self);

    /// For each $M_i$ in the $M$, it takes $F_i$, and adds it to $S_{M_i}$.
    ///
    /// Each $M_i \in [0, L_S)$, where $L_S$ is the length of the $S$.
    ///
    /// $L_M <= L_F, L_M <= L_S$, where $L_M$ it the length of the $M$, $L_F$ is the length of the $F$.
    ///
    /// | $S$    | $M$    | $F$    | $S$         |
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
    /// -   `from` — $F$ — the layer to add values to $S$ from.
    fn inverse_swizzle_add(&mut self, mapping: &[usize], from: &Self);
}

impl<T: PartialOrd + Bounded> Layer<T> for [T] {
    fn min_max(&self) -> (T, T) {
        let mut min: Self = T::MAX_BOUND;
        let mut max: Self = T::MIN_BOUND;

        for &i in self {
            if i < min {
                min = i;
            }

            if i > max {
                max = i;
            }
        }

        (min, max)
    }

    fn swizzle(&mut self, mapping: &[usize], from: &Self) {
        for (i, &m_i) in mapping.iter().enumerate() {
            self[i] = from[m_i];
        }
    }

    fn inverse_swizzle_add(&mut self, mapping: &[usize], from: &Self) {
        for (i, &m_i) in mapping.iter().enumerate() {
            self[m_i] += from[i];
        }
    }
}
