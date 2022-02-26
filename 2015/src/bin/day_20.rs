advent_of_code_2015::main!();

fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

fn part_1(input: usize) -> usize {
    let target = input / 10;
    let mut houses = vec![0; target + 1];
    for e in 1..=target {
        for h in (e..=target).step_by(e) {
            houses[h] += e;
        }
    }
    find_first(&houses, target)
}

fn part_2(target: usize) -> usize {
    let mut houses = vec![0; target + 1];
    for e in 1..=target {
        for h in (e..=target).step_by(e).take(50) {
            houses[h] += e * 11;
        }
    }
    find_first(&houses, target)
}

fn find_first(houses: &Vec<usize>, target: usize) -> usize {
    houses
        .iter()
        .enumerate()
        .find(|&(_, &h)| h >= target)
        .unwrap()
        .0
}
