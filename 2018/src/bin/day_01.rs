advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<i32>) -> i32 {
    input.into_iter().sum()
}

fn part_2(input: Vec<i32>) -> i32 {
    input
        .into_iter()
        .cycle()
        .fold_while((0, HashSet::new()), |(acc, mut v), n| {
            if v.insert(acc + n) {
                FoldWhile::Continue((acc + n, v))
            } else {
                FoldWhile::Done((acc + n, v))
            }
        })
        .into_inner()
        .0
}
