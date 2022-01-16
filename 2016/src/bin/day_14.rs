advent_of_code_2016::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> usize {
    solve(input, |s, n| to_array(md5::compute(format!("{}{}", s, n))))
}

fn part_2(input: &str) -> usize {
    solve(input, |s, n| {
        to_array(
            (0..2016).fold(md5::compute(format!("{}{}", s, n)), |acc, _| {
                md5::compute(format!("{:x}", acc))
            }),
        )
    })
}

fn solve(input: &str, gen: impl Fn(&str, usize) -> [u8; 32]) -> usize {
    let mut cache = HashMap::with_capacity(40_000);
    (0..64).fold(0, |i, _| {
        (i + 1..)
            .find(|&i| {
                cache
                    .entry(i)
                    .or_insert_with(|| gen(input, i))
                    .windows(3)
                    .find_map(|cs| cs.into_iter().all_equal().then(|| cs[0]))
                    .map(|char| {
                        ((i + 1)..=(i + 1000)).any(|j| {
                            cache
                                .entry(j)
                                .or_insert_with(|| gen(input, j))
                                .windows(5)
                                .any(|cs| cs.into_iter().all(|&c| c == char))
                        })
                    })
                    .unwrap_or(false)
            })
            .unwrap()
    })
}

fn to_array(digest: md5::Digest) -> [u8; 32] {
    digest
        .0
        .into_iter()
        .flat_map(|c| [c >> 4, c & 0xF])
        .collect_vec()
        .try_into()
        .unwrap()
}
