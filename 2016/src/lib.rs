#[macro_export]
macro_rules! main {
    () => {
        #[allow(dead_code)]
        use {
            advent_of_code_2016::Param,
            itertools::{iproduct, FoldWhile, Itertools},
            std::collections::{HashMap, HashSet},
            std::iter::{once, successors},
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

#[derive(Clone, Copy, Debug)]
pub enum Param {
    Value(i64),
    Var(usize),
    None,
}

impl std::str::FromStr for Param {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(n) = s.parse() {
            Self::Value(n)
        } else {
            Self::Var((s.as_bytes()[0] - b'a') as usize)
        })
    }
}

impl Param {
    pub fn from_generator(input: &str) -> Vec<(&str, Self, Self)> {
        input
            .lines()
            .map(|l| {
                let mut parts = l.split_whitespace();
                (
                    parts.next().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                    parts
                        .next()
                        .map(|s| s.parse().unwrap())
                        .unwrap_or(Param::None),
                )
            })
            .collect()
    }

    pub fn to_value(self, vars: &[i64; 4]) -> i64 {
        match self {
            Param::Value(n) => n,
            Param::Var(i) => vars[i],
            Param::None => 0,
        }
    }

    pub fn to_var(self, vars: &mut [i64; 4]) -> &mut i64 {
        match self {
            Param::Var(i) => &mut vars[i],
            _ => unreachable!(),
        }
    }
}
