advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<u32> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn part_1(input: Vec<u32>) -> usize {
    solve(input, 80)
}

fn part_2(input: Vec<u32>) -> usize {
    solve(input, 256)
}

fn solve(input: Vec<u32>, days: usize) -> usize {
    let mut fishes = input.into_iter().counts();
    for _ in 0..days {
        let zeros = fishes.remove(&0).unwrap_or(0);
        *fishes.entry(7).or_insert(0) += zeros;
        fishes.insert(9, zeros);
        for i in 1..=9 {
            let new = fishes.remove(&i).unwrap_or(0);
            fishes.insert(i - 1, new);
        }
    }
    fishes.values().sum()
}
