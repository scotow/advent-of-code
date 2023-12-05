advent_of_code_2023::main!();

fn generator(input: &str) -> (Vec<u64>, HashMap<&str, (&str, Vec<(Range<u64>, Range<u64>)>)>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    (seeds.trim_start_matches("seeds: ").split_whitespace().map(|n| n.parse().unwrap()).collect(),
        maps.split("\n\n").map(|m| {
            let (title, ranges) = m.split_once("\n").unwrap();
            let (from, to) = title.trim_end_matches(" map:").split_once("-to-").unwrap();
            (from, (to, ranges.lines().map(|l|{
                let (d, s, l) = l.split_whitespace().map(|n| n.parse().unwrap()).collect_tuple().unwrap();
                (s..s+l, d..d+l)
            }).collect()))
        }).collect()
    )
}

fn part_1((seeds, maps): (Vec<u64>, HashMap<&str, (&str, Vec<(Range<u64>, Range<u64>)>)>)) -> u64 {
    // dbg!((&seeds, &maps));

    seeds
        .into_iter()
        .map(|s| {
            let mut curr = ("seed", s);
            loop {
                let (next, ranges) = &maps[curr.0];
                curr.1 = ranges.iter()
                    .find(|(s, d)| s.contains(&curr.1))
                    .map(|(s, d)| d.start + (curr.1 - s.start))
                    .unwrap_or(curr.1);
                if *next == "location" {
                    return curr.1;
                } else {
                    curr.0 = next;
                }
            }
        })
        .min()
        .unwrap()
}

fn part_2((seeds, maps): (Vec<u64>, HashMap<&str, (&str, Vec<(Range<u64>, Range<u64>)>)>)) -> u64 {
    let seeds = seeds.into_iter().tuples().map(|(s, l)| s..s + l).collect_vec();

    // let mut curr = ("location", 500000);
    // loop {
    //     let (next, ranges) = maps.iter().find(|(k, (d, _))| *d == curr.0).unwrap();
    //     dbg!(&next, ranges, curr);
    //     // break;
    //     curr.1 = ranges.1.iter()
    //         .find(|(s, d)| d.contains(&curr.1))
    //         .map(|(s, d)| s.start + (curr.1 - d.start))
    //         .unwrap_or(curr.1);
    //     curr.0 = next;
    //     if *next == "seed" {
    //         dbg!(seeds.iter().any(|r| r.contains(&curr.1)).then_some(curr.1));
    //         // dbg!(curr);
    //         // return Some(0);
    //         return 0;
    //     } else {
    //         curr.0 = next;
    //     }
    // }


    (0..)
        .filter_map(|l| {
            let mut curr = ("location", l);
            loop {
                let (next, ranges) = maps.iter().find(|(k, (d, _))| *d == curr.0).unwrap();
                // dbg!(&next, ranges, curr);
                // break;
                curr.1 = ranges.1.iter()
                    .find(|(s, d)| d.contains(&curr.1))
                    .map(|(s, d)| s.start + (curr.1 - d.start))
                    .unwrap_or(curr.1);
                curr.0 = next;
                if *next == "seed" {
                    // seeds.iter().any(|r| r.contains(&curr.1)).then_some(curr.1);
                    return seeds.iter().any(|r| r.contains(&curr.1)).then_some(l);
                } else {
                    curr.0 = next;
                }
            }
        })
        .next().unwrap()


}
