#[macro_export]
macro_rules! main {
    () => {
        #[allow(unused_import)]
        use {
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
