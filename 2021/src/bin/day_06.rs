advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn part_1(input: Vec<usize>) -> usize {
    solve(input, 80)
}

fn part_2(input: Vec<usize>) -> usize {
    solve(input, 256)
}

fn solve(input: Vec<usize>, days: usize) -> usize {
    let mut fishes = [0; 9];
    input.into_iter().for_each(|f| fishes[f] += 1);
    (0..days).for_each(|_| {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    });
    fishes.iter().sum()
}
