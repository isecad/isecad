#![allow(dead_code)]

pub trait LayerCopy<T: Clone> {
    /// For each $M_i$ in the $M$, it takes $S_{F_i}$, and copies it to $S_i$.
    ///
    /// Each $M_i \in [0, L_F)$, where $L_F$ is the length of the $F$.
    ///
    /// $L_M <= L_S, L_M <= L_F$, where $L_M$ it the length of the $M$, $L_S$ is the length of the $S$.
    ///
    /// | $F$    | $M$    | $S$    |
    /// | :----: | :----: | :----: |
    /// | $8$    | $1$    | $1$    |
    /// | $1$    | $2$    | $4$    |
    /// | $4$    | $0$    | $8$    |
    /// | $8$    | $3$    | $8$    |
    /// | $9$    |        |        |
    ///
    /// # Arguments
    ///
    /// -   `self` — $S$ — the target layer.
    /// -   `indices` — $M$ — the mapping layer.
    /// -   `from` — $F$ — the source layer to copy items from.
    fn copy_from_slice_by_mapping(&mut self, mapping: &[usize], from: &Self);
}

impl<T: Copy> LayerCopy<T> for &mut [T] {
    fn copy_from_slice_by_mapping(&mut self, mapping: &[usize], from: &Self) {
        for (i, j) in mapping.iter().enumerate() {
            self[i] = from[*j];
        }
    }
}
