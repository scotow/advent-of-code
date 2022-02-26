advent_of_code_2020::main!();

fn generator(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_1(input: (u64, u64)) -> u64 {
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

fn part_2(_: (u64, u64)) -> &'static str {
    "N/A"
}
