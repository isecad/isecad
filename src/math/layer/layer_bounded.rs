#![allow(dead_code)]

use crate::*;
use std::cmp::*;

pub trait LayerBounded<T> {
    fn min_max(&self) -> (T, T);
}

impl<T: PartialOrd + Bounded> LayerBounded<T> for [T] {
    fn min_max(&self) -> (T, T) {
        let mut min = T::MAX_BOUND;
        let mut max = T::MIN_BOUND;

        for i in self {
            if *i < min {
                min = *i;
            }

            if *i > max {
                max = *i;
            }
        }

        (min, max)
    }
}
