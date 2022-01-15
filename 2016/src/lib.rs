#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            itertools::{iproduct, FoldWhile, Itertools},
            std::collections::{HashMap, HashSet},
            std::iter::successors,
            std::mem::replace,
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
