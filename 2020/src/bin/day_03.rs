advent_of_code_2020::main!();

fn generator(input: &str) -> &str {
    input
}
fn part_1(input: &str) -> usize {
    solve(input, (3, 1))
}

fn part_2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|s| solve(input, *s))
        .product()
}

fn solve(input: &str, shift: (usize, usize)) -> usize {
    let width = input.find('\n').unwrap();
    input
        .lines()
        .step_by(shift.1)
        .zip((0..width).cycle().step_by(shift.0))
        .filter(|&(l, x)| l.as_bytes()[x] == b'#')
        .count()
}
