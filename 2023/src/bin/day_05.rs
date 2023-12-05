advent_of_code_2023::main!();

fn generator(input: &str) -> (Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    (
        seeds
            .trim_start_matches("seeds: ")
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        maps.split("\n\n")
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|l| {
                        let (d, s, l) = l
                            .split_whitespace()
                            .map(|n| n.parse().unwrap())
                            .collect_tuple()
                            .unwrap();
                        (s..s + l, d..d + l)
                    })
                    .sorted_by_key(|(_s, d)| d.start)
                    .collect()
            })
            .collect(),
    )
}

fn part_1((seeds, maps): (Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>)) -> u64 {
    seeds
        .into_iter()
        .map(|s| {
            maps.iter().fold(s, |curr, ranges| {
                ranges
                    .iter()
                    .find(|(s, _)| s.contains(&curr))
                    .map(|(s, d)| d.start + (curr - s.start))
                    .unwrap_or(curr)
            })
        })
        .min()
        .unwrap()
}

fn part_2((seeds, maps): (Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>)) -> u64 {
    let seeds = seeds
        .into_iter()
        .tuples()
        .map(|(s, l)| s..s + l)
        .collect_vec();
    (0..)
        .filter_map(|l| {
            let rs = maps.iter().rev().fold(l, |curr, ranges| {
                ranges
                    .iter()
                    .find(|(_, d)| d.contains(&curr))
                    .map(|(s, d)| s.start + (curr - d.start))
                    .unwrap_or(curr)
            });
            seeds.iter().any(|s| s.contains(&rs)).then_some(l)
        })
        .next()
        .unwrap()
}
