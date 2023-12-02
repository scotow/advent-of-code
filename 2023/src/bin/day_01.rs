advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

fn part_1(input: Vec<&[u8]>) -> u32 {
    input.into_iter().map(|l| find(l, false) as u32).sum()
}

fn part_2(input: Vec<&[u8]>) -> u32 {
    input.into_iter().map(|l| find(l, true) as u32).sum()
}

fn find(input: &[u8], replace: bool) -> u8 {
    let iter = (0..input.len()).filter_map(|i| {
        if input[i].is_ascii_digit() {
            Some(input[i] - b'0')
        } else if replace {
            [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .into_iter()
            .enumerate()
            .find_map(|(j, n)| input[i..].starts_with(n.as_bytes()).then_some(j as u8 + 1))
        } else {
            None
        }
    });
    iter.clone().next().unwrap() * 10 + iter.rev().next().unwrap()
}
