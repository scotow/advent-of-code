advent_of_code_2025::main!();

fn generator(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|l| l[1..].parse::<i16>().unwrap() * (-1 + (&l[..1] == "R") as isize * 2) as i16)
        .collect()
}

fn part_1(input: Vec<i16>) -> usize {
    input
        .into_iter()
        .scan(50, |s, r| {
            *s = (*s + r) % 100;
            Some(*s)
        })
        .filter(|&s| s == 0)
        .count()
}

fn part_2(input: Vec<i16>) -> usize {
    input
        .into_iter()
        .scan(50, |s, r| {
            let ss = (0..r.abs()).map({
                let s = *s;
                move |i| (s + i * r.signum()) % 100
            });
            *s = (*s + r) % 100;
            Some(ss)
        })
        .flatten()
        .filter(|&s| s == 0)
        .count()
}
