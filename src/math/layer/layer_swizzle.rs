#![allow(dead_code)]

pub trait LayerSwizzle<T> {
    /// For each $M_i$ in the $M$, it takes $S_{F_i}$, and copies it to $S_i$.
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
}

impl<T: Copy> LayerSwizzle<T> for [T] {
    fn swizzle(&mut self, mapping: &[usize], from: &Self) {
        for (i, &j) in mapping.iter().enumerate() {
            self[i] = from[j];
        }
    }
}
