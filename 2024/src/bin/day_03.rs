advent_of_code_2024::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> u32 {
    solve(input, |_| true)
}

fn part_2(input: &str) -> u32 {
    let ranges = chain!(
        Some((0.., true)),
        Regex::new(r#"do(:?n't)?\(\)"#)
            .unwrap()
            .captures_iter(input)
            .map(|cap| (cap.get(0).unwrap().end().., &cap[0] == "do()"))
            .collect::<Vec<_>>()
    )
    .collect::<Vec<_>>();
    solve(input, |cap| {
        ranges
            .iter()
            .rev()
            .find_map(|(r, a)| r.contains(&cap.get(0).unwrap().start()).then_some(*a))
            .unwrap()
    })
}

fn solve(input: &str, f: impl Fn(&Captures) -> bool) -> u32 {
    Regex::new(r#"mul\((\d+),(\d+)\)"#)
        .unwrap()
        .captures_iter(input)
        .filter(f)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum()
}
