#![allow(dead_code)]

use crate::*;
use std::cmp::*;

pub trait LayerBounded<T> {
    fn min_max_element(&self) -> (&T, &T);
}

impl<T: PartialOrd + Bounded> LayerBounded<T> for [T] {
    fn min_max_element(&self) -> (&T, &T) {
        let mut min = T::max_bound();
        let mut max = T::min_bound();

        for i in self {
            if i < min {
                min = i;
            }

            if i > max {
                max = i;
            }
        }

        (min, max)
    }
}
