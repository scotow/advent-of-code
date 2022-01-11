#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            advent_of_code_2017::knot_hash,
            itertools::{iproduct, Itertools},
            std::collections::{HashMap, HashSet},
            std::iter::successors,
            std::mem::replace,
            std::str::FromStr,
        };

        fn main() {
            let input =
                generator(include_str!(concat!("../input/", module_path!(), ".txt")).trim());
            println!("{}", part_1(input.clone()));
            println!("{}", part_2(input));
        }
    };
}

pub mod knot_hash {
    use itertools::Itertools;

    pub fn knot(numbers: &mut [u8; 256], lens: &[usize], ptr: &mut usize, skip: &mut usize) {
        for &len in lens {
            let to_rev = numbers
                .iter()
                .cycle()
                .skip(*ptr)
                .take(len)
                .copied()
                .collect_vec();
            for (i, r_i) in (*ptr..*ptr + len).map(|i| i % 256).rev().enumerate() {
                numbers[r_i] = to_rev[i];
            }
            *ptr += len + *skip;
            *skip += 1;
        }
    }

    pub fn hash(input: &str) -> [u8; 16] {
        let mut numbers = (0..=255).collect_vec().try_into().unwrap();
        let input = input
            .bytes()
            .map(|c| c as usize)
            .chain([17, 31, 73, 47, 23])
            .collect_vec();
        let mut ptr = 0;
        let mut skip = 0;
        for _ in 0..64 {
            knot(&mut numbers, &input, &mut ptr, &mut skip);
        }
        numbers
            .chunks(16)
            .map(|chunk| chunk.into_iter().copied().reduce(|acc, n| acc ^ n).unwrap())
            .collect_vec()
            .try_into()
            .unwrap()
    }
}
