advent_of_code_2024::main!();

fn generator(input: &str) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let (l, r) = input.split_once("\n\n").unwrap();
    (
        l.lines()
            .map(|l| {
                l.split('|')
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .into_group_map(),
        r.lines()
            .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
            .collect(),
    )
}

fn part_1((rules, updates): (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>)) -> u32 {
    updates.iter().filter_map(|u| solve(&rules, u)).sum()
}

fn part_2((rules, updates): (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>)) -> u32 {
    updates
        .into_iter()
        .filter(|u| solve(&rules, u).is_none())
        .map(|mut update| {
            let target = update.len() / 2 + 1;
            while !update.is_empty() {
                let (i, n) = update
                    .iter()
                    .enumerate()
                    .find_map(|(i, &u)| {
                        update
                            .iter()
                            .all(|&u4| {
                                u4 == u || rules.get(&u4).map(|us| us.contains(&u)).unwrap_or(false)
                            })
                            .then_some((i, u))
                    })
                    .unwrap();
                if update.len() == target {
                    return n as u32;
                } else {
                    update.remove(i);
                }
            }
            unreachable!()
        })
        .sum()
}

fn solve(rules: &HashMap<u8, Vec<u8>>, update: &[u8]) -> Option<u32> {
    update
        .iter()
        .enumerate()
        .all(|(i, n)| {
            rules
                .get(n)
                .map(|o| o.iter().all(|o| !update[..i].contains(o)))
                .unwrap_or(true)
        })
        .then_some(update[update.len() / 2] as u32)
}
