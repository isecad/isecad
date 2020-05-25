#![allow(dead_code)]

use super::*;

pub trait LayerBounded<T: Ord + Bounded<T>> {
    fn min_index(&self) -> usize;
    fn max_index(&self) -> usize;

    fn min_element(&self) -> &T;
    fn max_element(&self) -> &T;
}

impl<T: Ord + Bounded<T>> LayerBounded<T> for &[T] {
    fn min_index(&self) -> usize {
        let mut min: &T = &T::MAX_BOUND;
        let mut min_index: usize = 0;

        for (i, value) in self.iter().enumerate() {
            if value < min {
                min = value;
                min_index = i;
            }
        }

        min_index
    }

    fn max_index(&self) -> usize {
        let mut max: &T = &T::MIN_BOUND;
        let mut max_index: usize = 0;

        for (i, value) in self.iter().enumerate() {
            if value > max {
                max = value;
                max_index = i;
            }
        }

        max_index
    }

    fn min_element(&self) -> &T {
        &self[self.min_index()]
    }

    fn max_element(&self) -> &T {
        &self[self.max_index()]
    }
}
