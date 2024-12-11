advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn part_1(stones: Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    stones
        .into_iter()
        .map(|stone| solve(stone, 25, &mut cache))
        .sum()
}

fn part_2(stones: Vec<u64>) -> u64 {
    let mut cache = HashMap::new();
    stones
        .into_iter()
        .map(|stone| solve(stone, 75, &mut cache))
        .sum()
}

fn solve(stone: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(res) = cache.get(&(stone, blinks)) {
        return *res;
    }
    let next = if stone == 0 {
        [Some(1), None]
    } else if stone.ilog10() % 2 == 1 {
        let power = 10u64.pow(stone.ilog10() / 2 + 1);
        [Some(stone / power), Some(stone % power)]
    } else {
        [Some(stone * 2024), None]
    };
    let res = next
        .into_iter()
        .filter_map(identity)
        .map(|stone| solve(stone, blinks - 1, cache))
        .sum();
    cache.insert((stone, blinks), res);
    res
}
