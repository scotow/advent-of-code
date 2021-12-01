#[macro_export]
macro_rules! input {
    () => {
        std::fs::read_to_string(concat!("src/input/", module_path!(), ".txt")).unwrap()
    };
}

#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use itertools::Itertools;
        
        fn main() {
            let input = generator(advent_of_code_2021::input!());
            println!("{}\n{}", part_1(input.clone()), part_2(input));
        }
    };
}
