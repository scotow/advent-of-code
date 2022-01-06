#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            itertools::{iproduct, Itertools},
            std::collections::{HashMap, HashSet},
        };

        fn main() {
            let input =
                generator(include_str!(concat!("../input/", module_path!(), ".txt")).trim());
            println!("{}", part_1(input.clone()));
            println!("{}", part_2(input));
        }
    };
}
