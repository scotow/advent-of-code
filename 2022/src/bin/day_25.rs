advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_1(input: Vec<&str>) -> String {
    unparse(input.into_iter().map(parse).sum())
}

fn part_2(_: Vec<&str>) -> &'static str {
    "N/A"
}

fn parse(n: &str) -> i64 {
    n.bytes()
        .rev()
        .enumerate()
        .map(|(i, b)| match b {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!(),
        } * 5i64.pow(i as u32))
        .sum()
}

fn unparse(mut n: i64) -> String {
    let mut s = Vec::new();
    while n > 0 {
        s.push(match n % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        });
        n = (n + 2) / 5;
    }
    s.into_iter().rev().collect()
}
