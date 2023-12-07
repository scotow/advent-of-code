advent_of_code_2023::main!();

type Hand = [u8; 5];

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    High,
    Pair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}

impl From<Hand> for HandType {
    fn from(hand: Hand) -> Self {
        let map = hand.into_iter().counts();
        match map.len() {
            1 => Self::Five,
            2 if map.values().any(|&n| n == 4) => Self::Four,
            2 => Self::Full,
            3 if map.values().any(|&n| n == 3) => Self::Three,
            3 => Self::TwoPairs,
            4 => Self::Pair,
            5 => Self::High,
            _ => unreachable!(),
        }
    }
}

fn generator(input: &str) -> Vec<(Hand, u32)> {
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

fn part_1(input: Vec<(Hand, u32)>) -> u32 {
    score(input.iter().map(|h| (HandType::from(h.0), h.0, h.1)))
}

fn part_2(input: Vec<(Hand, u32)>) -> u32 {
    score(input.into_iter().map(|(mut h, b)| {
        h = h.map(|c| (c == 11).then_some(1).unwrap_or(c));
        (
            h.into_iter()
                .map(|c| if c == 1 { 2..=14 } else { c..=c })
                .multi_cartesian_product()
                .map(|h| HandType::from(Hand::try_from(h).unwrap()))
                .max()
                .unwrap(),
            h,
            b,
        )
    }))
}

fn score<I: Iterator<Item = (HandType, Hand, u32)>>(iter: I) -> u32 {
    iter.sorted()
        .enumerate()
        .map(|(i, (_, _, b))| (i as u32 + 1) * b)
        .sum()
}
