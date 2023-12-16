advent_of_code_2023::main!();

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Spring {
    Ok,
    Damaged,
    Unknown,
}

fn generator(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (ss, ns) = l.split_once(' ').unwrap();
            (
                ss.bytes()
                    .map(|b| match b {
                        b'.' => Spring::Ok,
                        b'#' => Spring::Damaged,
                        b'?' => Spring::Unknown,
                        _ => unreachable!(),
                    })
                    .collect(),
                ns.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part_1(input: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(m, p)| solve(&m, &p, 0, &mut HashMap::new()))
        .sum()
}

fn part_2(input: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(m, p)| {
            solve(
                &Itertools::intersperse(&mut repeat_n(m, 5), vec![Spring::Unknown])
                    .flatten()
                    .collect::<Vec<_>>(),
                &repeat_n(p, 5).flatten().collect::<Vec<_>>(),
                0,
                &mut HashMap::new(),
            )
        })
        .sum()
}

fn solve(
    map: &[Spring],
    pattern: &[usize],
    damaged: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(cached) = cache.get(&(map.len(), pattern.len(), damaged)) {
        return *cached;
    }

    if map.is_empty() {
        return (pattern.is_empty() && damaged == 0 || pattern.len() == 1 && damaged == pattern[0])
            as usize;
    }

    let process_ok = |cache| {
        if damaged == 0 {
            solve(&map[1..], pattern, 0, cache)
        } else if !pattern.is_empty() && damaged == pattern[0] {
            solve(&map[1..], &pattern[1..], 0, cache)
        } else {
            0
        }
    };
    let process_damaged = |cache| solve(&map[1..], pattern, damaged + 1, cache);

    let res = match map[0] {
        Spring::Ok => process_ok(cache),
        Spring::Damaged => process_damaged(cache),
        Spring::Unknown => process_ok(cache) + process_damaged(cache),
    };
    cache.insert((map.len(), pattern.len(), damaged), res);
    res
}
