#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            advent_of_code_2018::abs_diff,
            itertools::{iproduct, FoldWhile, Itertools},
            std::collections::{HashMap, HashSet},
            std::fmt::{Debug, Display, Formatter},
            std::iter::{once, successors},
            std::mem::replace,
            std::ops::{Range, RangeInclusive},
            std::str::FromStr,
        };

        fn main() {
            let input =
                generator(include_str!(concat!("../input/", module_path!(), ".txt")).trim_end());
            println!("{}", part_1(input.clone()));
            println!("{}", part_2(input));
        }
    };
}

use std::ops::Sub;

pub fn abs_diff<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}
