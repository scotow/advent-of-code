advent_of_code_2025::main!();

fn generator(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    (
        ranges
            .lines()
            .map(|l| {
                let (f, t) = l
                    .split('-')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                f..=t
            })
            .collect(),
        ids.lines().map(|l| l.parse().unwrap()).collect(),
    )
}

fn part_1((ranges, ids): (Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
    ids.into_iter()
        .filter(|&id| ranges.iter().any(|r| r.contains(&id)))
        .count()
}

fn part_2((mut ranges, _): (Vec<RangeInclusive<u64>>, Vec<u64>)) -> usize {
    ranges.sort_by_key(|r| (*r.start(), *r.end()));
    let mut n = 0;
    let mut current = ranges[0].clone();
    for r in ranges.into_iter().skip(1) {
        let (sr1, sr2) = if current.contains(r.start()) {
            (*current.start()..=(*current.end()).max(*r.end()), None)
        } else {
            (current, Some(r))
        };
        if let Some(sr2) = sr2 {
            n += sr1.count();
            current = sr2;
        } else {
            current = sr1;
        }
    }
    n + current.count()
}
