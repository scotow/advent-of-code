#[macro_export]
macro_rules! main {
    () => {
        #[allow(unused_import)]
        use {
            advent_of_code_2022::{deltas4, deltas8, neighbors4, neighbors8},
            itertools::{chain, iproduct, repeat_n, FoldWhile, Itertools},
            std::cmp::Ordering,
            std::collections::{HashMap, HashSet, VecDeque},
            std::fmt::{Debug, Display, Formatter},
            std::iter::zip,
            std::iter::{once, successors},
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

use itertools::iproduct;
use num::iter::range_inclusive;
use num::{One, ToPrimitive, Zero};
use std::ops::{Add, Neg, Sub};

pub fn deltas4<N>() -> impl Iterator<Item = (N, N)>
where
    N: Add<N, Output = N>
        + Sub<N, Output = N>
        + Neg<Output = N>
        + PartialOrd<N>
        + One
        + Zero
        + Copy
        + ToPrimitive
        + 'static,
{
    deltas8::<N>().filter(|&(x, y)| x.is_zero() || y.is_zero())
}

pub fn deltas8<N>() -> impl Iterator<Item = (N, N)>
where
    N: Add<N, Output = N>
        + Sub<N, Output = N>
        + Neg<Output = N>
        + PartialOrd<N>
        + One
        + Zero
        + Copy
        + ToPrimitive
        + 'static,
{
    iproduct!(
        range_inclusive(N::one().neg(), N::one()),
        range_inclusive(N::one().neg(), N::one())
    )
    .filter(|&(x, y)| !(x.is_zero() && y.is_zero()))
}

pub fn neighbors4<N>(x: N, y: N) -> impl Iterator<Item = (N, N)>
where
    N: Add<N, Output = N>
        + Sub<N, Output = N>
        + Neg<Output = N>
        + PartialOrd<N>
        + One
        + Zero
        + Copy
        + ToPrimitive
        + 'static,
{
    deltas4::<N>().map(move |(dx, dy)| (dx + x, dy + y))
}

pub fn neighbors8<N>(x: N, y: N) -> impl Iterator<Item = (N, N)>
where
    N: Add<N, Output = N>
        + Sub<N, Output = N>
        + Neg<Output = N>
        + PartialOrd<N>
        + One
        + Zero
        + Copy
        + ToPrimitive
        + 'static,
{
    deltas8::<N>().map(move |(dx, dy)| (dx + x, dy + y))
}
