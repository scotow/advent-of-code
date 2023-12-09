advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn part_1(input: Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .map(|n| solve(n, <[i32]>::last, i32::add))
        .sum()
}

fn part_2(input: Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .map(|n| solve(n, <[i32]>::first, i32::sub))
        .sum()
}

fn solve(input: Vec<i32>, side: fn(&[i32]) -> Option<&i32>, op: fn(i32, i32) -> i32) -> i32 {
    if input.iter().all_equal() {
        input[0]
    } else {
        op(
            *side(&input).unwrap(),
            solve(input.windows(2).map(|ns| ns[1] - ns[0]).collect(), side, op),
        )
    }
}
