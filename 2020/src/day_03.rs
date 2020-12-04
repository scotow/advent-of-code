fn solve(input: &str, shift: (usize, usize)) -> usize {
    let width = input.find('\n').unwrap();
    input.lines()
        .step_by(shift.1)
        .zip((0..width).cycle().step_by(shift.0))
        .filter(|&(l, x)| l.as_bytes()[x] == b'#')
        .count()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    solve(input, (3, 1))
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|s| solve(input, *s))
        .product()
}
