advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn part_1(mut input: Vec<i32>) -> i32 {
    input.sort();
    let med = input[input.len() / 2];
    input.into_iter().map(|n| (med - n).abs()).sum()
}

fn part_2(input: Vec<i32>) -> (i32, i32) {
    let mean = input.iter().sum::<i32>() / input.len() as i32;
    let solve = |m: i32| {
        input
            .iter()
            .map(|n| {
                let d = (m - n).abs();
                (d * (d + 1)) / 2
            })
            .sum()
    };
    (solve(mean + 1), solve(mean))
}
