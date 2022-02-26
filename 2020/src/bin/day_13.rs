advent_of_code_2020::main!();

fn generator(input: &str) -> (u64, Vec<(usize, u64)>) {
    let (dep, busses) = input.lines().collect_tuple().unwrap();
    (
        dep.parse().unwrap(),
        busses
            .split(',')
            .enumerate()
            .filter_map(|(i, b)| b.parse().map(|b| (i, b)).ok())
            .collect(),
    )
}

fn part_1(input: (u64, Vec<(usize, u64)>)) -> u64 {
    let d = iproduct!(input.0.., input.1.iter())
        .find(|&(t, &(_, b))| t % b == 0)
        .unwrap();
    d.1 .1 * (d.0 - input.0)
}

fn part_2(input: (u64, Vec<(usize, u64)>)) -> u64 {
    let mut timestamp = 0;
    let mut shift = input.1[0].1;
    for &(i, b) in &input.1[1..] {
        while (timestamp + i as u64) % b != 0 {
            timestamp += shift;
        }
        shift *= b;
    }

    timestamp
}
