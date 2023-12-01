advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.bytes().collect()).collect()
}

fn part_1(input: Vec<Vec<u8>>) -> u32 {
    input
        .into_iter()
        .map(|mut l| (find(&mut l, false, true) + find(&mut l, false, false)) as u32)
        .sum()
}

fn part_2(input: Vec<Vec<u8>>) -> u32 {
    input
        .into_iter()
        .map(|mut l| (find(&mut l, true, true) + find(&mut l, true, false)) as u32)
        .sum()
}

fn find(input: &mut Vec<u8>, replace: bool, dir: bool) -> u8 {
    if !dir {
        input.reverse();
    }
    for i in 0..input.len() {
        if input[i].is_ascii_digit() {
            return (input[i] - b'0') * (1 + dir as u8 * 9);
        }
        if replace {
            for (j, n) in [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .into_iter()
            .map(|s| {
                dir.then(|| s.as_bytes().to_owned())
                    .unwrap_or_else(|| s.bytes().rev().collect())
            })
            .enumerate()
            {
                if input[i..].starts_with(&n) {
                    return (j + 1) as u8 * (1 + dir as u8 * 9);
                }
            }
        }
    }
    unreachable!()
}
