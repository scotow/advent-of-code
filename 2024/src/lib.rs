#[macro_export]
macro_rules! main {
    () => {
        #[allow(unused_import)]
        use {
            advent_of_code_2024::{
                deltas4, deltas4diag, deltas8, max, neighbors4, neighbors4diag, neighbors6, neighbors8, Pos, Pos3,
            },
            itertools::{chain, iproduct, izip, repeat_n, Either, FoldWhile, Itertools},
            num::integer::{div_ceil, lcm, Integer},
            pathfinding::directed::{
                bfs::{bfs, bfs_reach},
                dfs::{dfs, dfs_reach},
                dijkstra::{build_path as dijkstra_path, dijkstra, dijkstra_all},
                count_paths::count_paths,
            },
            std::cmp::Ordering,
            std::collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
            std::convert::identity,
            std::fmt::{Debug, Display, Formatter},
            std::hash::{Hash, Hasher},
            std::iter::{once, successors, zip, Peekable},
            std::mem::{replace, take, swap},
            std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Range, RangeInclusive, Sub},
            std::rc::Rc,
            std::str::{from_utf8, FromStr},
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
use num::traits::{WrappingAdd, WrappingSub};
use num::{One, ToPrimitive, Zero};
use std::ops::{Add, Neg, Sub};

pub type Pos<T> = (T, T);
pub type Pos3<T> = (T, T, T);

pub fn deltas4<N>() -> impl Iterator<Item = (N, N)>
where
    N: Neg<Output = N> + Zero + One,
{
    [
        (N::zero(), N::one().neg()),
        (N::one(), N::zero()),
        (N::zero(), N::one()),
        (N::one().neg(), N::zero()),
    ] // ← ↓ → ↑
    .into_iter()
}

pub fn deltas4diag<N>() -> impl Iterator<Item = (N, N)>
where
    N: Neg<Output = N> + Zero + One,
{
    [
        (N::one().neg(), N::one().neg()),
        (N::one().neg(), N::one()),
        (N::one(), N::one()),
        (N::one(), N::one().neg()),
    ] // ↖ ↗ ↘ ↙
        .into_iter()
}

pub fn deltas8<N>() -> impl Iterator<Item = (N, N)>
where
    N: Add<N, Output = N>
        + Sub<N, Output = N>
        + Neg<Output = N>
        + PartialOrd<N>
        + Zero
        + One
        + ToPrimitive
        + Copy,
{
    iproduct!(
        range_inclusive(N::one().neg(), N::one()),
        range_inclusive(N::one().neg(), N::one())
    )
    .filter(|&(x, y)| !(x.is_zero() && y.is_zero()))
}

pub fn neighbors4<N>(x: N, y: N) -> impl Iterator<Item = (N, N)>
where
    N: WrappingAdd + WrappingSub + PartialOrd<N> + One + ToPrimitive + Copy,
{
    neighbors8(x, y).filter(move |&(xn, yn)| x == xn || y == yn)
}

pub fn neighbors4diag<N>(x: N, y: N) -> impl Iterator<Item = (N, N)>
where
    N: WrappingAdd + WrappingSub + PartialOrd<N> + One + ToPrimitive + Copy,
{
    neighbors8(x, y).filter(move |&(xn, yn)| x != xn && y != yn)
}

pub fn neighbors8<N>(x: N, y: N) -> impl Iterator<Item = (N, N)>
where
    N: WrappingAdd + WrappingSub + PartialOrd<N> + One + ToPrimitive + Copy,
{
    iproduct!(
        [x.wrapping_sub(&N::one()), x, x.wrapping_add(&N::one())],
        [y.wrapping_sub(&N::one()), y, y.wrapping_add(&N::one())]
    ) // ↖ ← ↙ ↑ ↓ ↗ → ↘
    .filter(move |&xyn| xyn != (x, y))
}

pub fn neighbors6<N>(x: N, y: N, z: N) -> impl Iterator<Item = (N, N, N)>
where
    N: WrappingAdd + WrappingSub + One + Copy,
{
    [
        (x.wrapping_sub(&N::one()), y, z),
        (x.wrapping_add(&N::one()), y, z),
        (x, y.wrapping_sub(&N::one()), z),
        (x, y.wrapping_add(&N::one()), z),
        (x, y, z.wrapping_sub(&N::one())),
        (x, y, z.wrapping_add(&N::one())),
    ]
    .into_iter()
}

#[macro_export]
macro_rules! max {
    ( $a:expr, $( $b:expr ),* ) => {
        $a$(.max($b))*
    };
}
