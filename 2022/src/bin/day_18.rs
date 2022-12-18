advent_of_code_2022::main!();

fn generator(input: &str) -> HashSet<(i16, i16, i16)> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: HashSet<(i16, i16, i16)>) -> usize {
    input
        .iter()
        .map(|&(x, y, z)| neighbors6(x, y, z).filter(|n| !input.contains(n)).count())
        .sum()
}

fn part_2(input: HashSet<(i16, i16, i16)>) -> usize {
    let (min_x, max_x) = input.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (min_y, max_y) = input.iter().map(|p| p.1).minmax().into_option().unwrap();
    let (min_z, max_z) = input.iter().map(|p| p.2).minmax().into_option().unwrap();
    bfs_reach((min_x, min_y, min_z), |&(x, y, z)| {
        neighbors6(x, y, z).filter(|&(x, y, z)| {
            !input.contains(&(x, y, z))
                && (min_x - 1..=max_x + 1).contains(&x)
                && (min_y - 1..=max_y + 1).contains(&y)
                && (min_z - 1..=max_z + 1).contains(&z)
        })
    })
    .map(|(x, y, z)| neighbors6(x, y, z).filter(|n| input.contains(n)).count())
    .sum()
}
