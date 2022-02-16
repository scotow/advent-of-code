#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            advent_of_code_2019::{abs_diff, intcode::*, m_dist},
            itertools::{chain, iproduct, FoldWhile, Itertools},
            num_integer::Integer,
            std::cmp::Ordering,
            std::collections::{HashMap, HashSet, VecDeque},
            std::fmt::{Debug, Display, Formatter},
            std::iter::{once, successors},
            std::mem::replace,
            std::ops::{Add, Sub},
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

pub mod intcode;

use std::ops::Sub;

#[macro_export]
macro_rules! m_dist {
    ($($e:expr),*) => {
        {
            let mut d = 0;
            $(
                d += $e.abs();
            )*
            d
        }
    };
    ($($e:expr),*; $($o:expr),*) => {
        {
            let mut d = 0;
            $(
                d += abs_diff($e, $o);
            )*
            d
        }
    }
}

pub fn abs_diff<T: Sub<Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}
