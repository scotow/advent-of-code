advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<([u8; 5], u32)> {
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            (
                hand.bytes()
                    .map(|b| match b {
                        b'2'..=b'9' => b - b'0',
                        b'T' => 10,
                        b'J' => 11,
                        b'Q' => 12,
                        b'K' => 13,
                        b'A' => 14,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid.parse().unwrap(),
            )
        })
        .collect()
}

fn part_1(input: Vec<([u8; 5], u32)>) -> u32 {
    score(input.iter().map(|h| (strength(h.0), h.0, h.1)))
}

fn part_2(input: Vec<([u8; 5], u32)>) -> u32 {
    score(input.into_iter().map(|(mut h, b)| {
        h = h.map(|c| if c == 11 { 1 } else { c });
        (
            h.into_iter()
                .map(|c| if c == 1 { 2..=14 } else { c..=c })
                .multi_cartesian_product()
                .map(|h| strength(h.try_into().unwrap()))
                .max()
                .unwrap(),
            h,
            b,
        )
    }))
}

fn strength(hand: [u8; 5]) -> u8 {
    let map = hand.into_iter().counts();
    match map.len() {
        1 => 6,
        2 if map.values().any(|&n| n == 4) => 5,
        2 => 4,
        3 if map.values().any(|&n| n == 3) => 3,
        3 => 2,
        4 => 1,
        5 => 0,
        _ => unreachable!(),
    }
}

fn score<I: Iterator<Item = (u8, [u8; 5], u32)>>(iter: I) -> u32 {
    iter.sorted()
        .enumerate()
        .map(|(i, (_, _, b))| (i as u32 + 1) * b)
        .sum()
}
