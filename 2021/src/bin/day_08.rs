use std::collections::HashMap;

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|l| {
            let (i, o) = l.split_once(" | ").unwrap();
            (i.split(' ').map(|s| s.to_owned()).collect(), o.split(' ').map(|s| s.to_owned()).collect())
        })
        .collect()
}

fn part_1(input: Vec<(Vec<String>, Vec<String>)>) -> usize {
    input
        .into_iter()
        .flat_map(|l| l.1)
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part_2(input: Vec<(Vec<String>, Vec<String>)>) -> i32 {
    let mapping = [
        "abcdeg",
        "ab",
        "acdfg",
        "abcdf",
        "abef",
        "bcdef",
        "bcdefg",
        "abd",
        "abcdefg",
        "abcdef",
    ].into_iter()
    .enumerate()
    .map(|(i, n)| (n, i))
    .collect::<HashMap<_, _>>();
    dbg!(mapping);
    
    dbg!(
        input
            .into_iter()
            .map(|(_i, o)| {
                o
                    .into_iter()
                    .map(|mut s| {
                        s.chars().sorted().collect::<String>()
                    })
                    .collect_vec()
            })
            .collect_vec()
    );

    0
}
