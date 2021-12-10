advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.as_bytes().into_iter().copied().collect())
        .collect()
}

fn part_1(input: Vec<Vec<u8>>) -> u32 {
    input
        .into_iter()
        .map(|l| corrupt_score(&l).unwrap_or(0))
        .sum()
}

fn part_2(input: Vec<Vec<u8>>) -> u64 {
    let res = input
        .into_iter()
        .filter(|l| corrupt_score(&l).is_none())
        .map(|l| {
            let mut stack = Vec::new();
            for c in l {
                if matches!(c, b'(' | b'[' | b'{' | b'<') {
                    stack.push(c);
                } else {
                    stack.pop();
                }
            }
            stack.into_iter().rev().fold(0, |n, c| {
                n * 5
                    + match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                }
            })
        })
        .sorted()
        .collect_vec();
    res[res.len() / 2]
}

fn corrupt_score(input: &[u8]) -> Option<u32> {
    let mut stack = Vec::new();
    for &c in input {
        if matches!(c, b'(' | b'[' | b'{' | b'<') {
            stack.push(c);
        } else {
            let latest = stack.pop().unwrap();
            if c.wrapping_sub(latest) > 2 {
                return Some(match c {
                    b')' => 3,
                    b']' => 57,
                    b'}' => 1197,
                    b'>' => 25137,
                    _ => unreachable!(),
                });
            }
        }
    }
    None
}