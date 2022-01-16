advent_of_code_2016::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> usize {
    solve(input, |s, n| {
        format!("{:x}", md5::compute(format!("{}{}", s, n)))
    })
}

fn part_2(input: &str) -> usize {
    solve(input, |s, n| {
        format!(
            "{:x}",
            (0..2016).fold(md5::compute(format!("{}{}", s, n)), |acc, _| {
                md5::compute(format!("{:x}", acc))
            })
        )
    })
}

fn solve(input: &str, gen: impl Fn(&str, usize) -> String) -> usize {
    let mut cache = HashMap::new();
    (0..64).fold(0, |i, _| {
        (i + 1..)
            .find(|&i| {
                cache
                    .entry(i)
                    .or_insert_with(|| gen(input, i))
                    .as_bytes()
                    .windows(3)
                    .find_map(|cs| cs.into_iter().all_equal().then(|| cs[0]))
                    .map(|char| {
                        ((i + 1)..=(i + 1000)).any(|j| {
                            cache
                                .entry(j)
                                .or_insert_with(|| gen(input, j))
                                .as_bytes()
                                .windows(5)
                                .any(|cs| cs.into_iter().all(|&c| c == char))
                        })
                    })
                    .unwrap_or(false)
            })
            .unwrap()
    })
}
