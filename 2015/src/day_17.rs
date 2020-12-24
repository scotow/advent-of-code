use itertools::Itertools;

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<u16> {
    input.lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day17, part1)]
fn part1(containers: &[u16]) -> usize {
    containers.iter().map(|&c| (0..=c).step_by(c as usize))
        .multi_cartesian_product()
        .filter(|cs| cs.iter().sum::<u16>() == 150)
        .count()
}

#[aoc(day17, part2)]
fn part2(containers: &[u16]) -> usize {
    let min = containers.iter().map(|&c| (0..=c).step_by(c as usize))
        .multi_cartesian_product()
        .filter(|cs| cs.iter().sum::<u16>() == 150)
        .map(|cs| cs.iter().filter(|&&c| c != 0).count())
        .min().unwrap();

    containers.iter().map(|&c| (0..=c).step_by(c as usize))
        .multi_cartesian_product()
        .filter(|cs| cs.iter().sum::<u16>() == 150)
        .filter(|cs| cs.iter().filter(|&&c| c != 0).count() == min)
        .count()
}
