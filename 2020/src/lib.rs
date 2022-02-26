#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            core::cmp::Ordering,
            itertools::{chain, iproduct, FoldWhile, Itertools},
            std::collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
            std::fmt::{Debug, Display, Formatter},
            std::hash::{Hash, Hasher},
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
