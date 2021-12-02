#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use itertools::Itertools;

        fn main() {
            let input = generator(include_str!(concat!("../input/", module_path!(), ".txt")));
            println!("{}", part_1(input.clone()));
            println!("{}", part_2(input));
        }
    };
}
