use itertools::Itertools;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (u64, u64) {
    input.lines().map(|l| l.parse().unwrap()).collect_tuple().unwrap()
}

#[aoc(day25, part1)]
pub fn part1(input: &(u64, u64)) -> u64 {
    let mut pkc = 1;
    let mut enc = 1;
    loop {
        pkc = pkc * 7 % 20201227;
        enc = enc * input.1 % 20201227;
        if pkc == input.0 {
            return enc;
        }
    }
}
