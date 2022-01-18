advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (m, n) = l
                .split(&[' ', '.'])
                .skip(3)
                .step_by(8)
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            ((n + i + 1) % m, m)
        })
        .collect()
}

fn part_1(input: Vec<(usize, usize)>) -> usize {
    solve(input)
}

fn part_2(mut input: Vec<(usize, usize)>) -> usize {
    input.push((input.len() + 1, 11));
    solve(input)
}

fn solve(discs: Vec<(usize, usize)>) -> usize {
    let (x, m) = discs
        .into_iter()
        .reduce(|(mut x, mut m), (y, n)| {
            while x % n != y {
                x += m;
            }
            m = num_integer::lcm(m, n);
            (x, m)
        })
        .unwrap();
    m - x
}
