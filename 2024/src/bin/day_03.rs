advent_of_code_2024::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> u32 {
    solve(input)
}

fn part_2(input: &str) -> u32 {
    format!("do(){input}")
        .split("do()")
        .map(|p| solve(p.split("don't()").next().unwrap()))
        .sum()
}

fn solve(input: &str) -> u32 {
    input
        .split("mul(")
        .flat_map(|p| {
            p.split(')').next().map(|p| {
                p.split(',')
                    .map(|n| n.parse().unwrap_or(0))
                    .pad_using(2, |_| 0)
                    .product::<u32>()
            })
        })
        .sum()
}
