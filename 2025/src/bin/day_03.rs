advent_of_code_2025::main!();

fn generator(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part_1(input: Vec<&[u8]>) -> u64 {
    input.into_iter().map(|b| find(b, 1)).sum()
}

fn part_2(input: Vec<&[u8]>) -> u64 {
    input.into_iter().map(|b| find(b, 11)).sum()
}

fn find(input: &[u8], r: usize) -> u64 {
    let mut max = (0, 0);
    for (i, &b) in input[..input.len() - r].iter().enumerate() {
        if b > max.1 {
            max = (i, b);
        }
    }
    if r == 0 {
        (max.1 - b'0') as u64
    } else {
        (max.1 - b'0') as u64 * 10u64.pow(r as u32) + find(&input[max.0 + 1..], r - 1)
    }
}
