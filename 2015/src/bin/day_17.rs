advent_of_code_2015::main!();

fn generator(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(containers: Vec<u16>) -> usize {
    matching(containers).count()
}

fn part_2(containers: Vec<u16>) -> usize {
    let mut mapping = HashMap::new();
    matching(containers)
        .map(|cs| cs.iter().filter(|&&c| c != 0).count())
        .for_each(|n| *mapping.entry(n).or_insert(0) += 1);

    *mapping.iter().min().unwrap().1
}

fn matching(containers: Vec<u16>) -> impl Iterator<Item = Vec<u16>> {
    containers
        .iter()
        .map(|&c| (0..=c).step_by(c as usize))
        .multi_cartesian_product()
        .filter(|cs| cs.iter().sum::<u16>() == 150)
}
