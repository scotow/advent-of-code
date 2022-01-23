advent_of_code_2018::main!();

fn generator(input: &str) -> HashMap<u8, Vec<u8>> {
    input
        .lines()
        .map(|l| {
            let p = l.split_whitespace().collect_vec();
            (p[7].as_bytes()[0], p[1].as_bytes()[0])
        })
        .into_group_map()
}

fn part_1(input: HashMap<u8, Vec<u8>>) -> String {
    solve::<1>(input).0
}

fn part_2(input: HashMap<u8, Vec<u8>>) -> usize {
    solve::<5>(input).1
}

fn solve<const W: usize>(input: HashMap<u8, Vec<u8>>) -> (String, usize) {
    let mut queue = input
        .values()
        .flatten()
        .collect::<HashSet<_>>()
        .difference(&input.keys().collect::<HashSet<_>>())
        .map(|&&n| n)
        .collect_vec();
    let mut workers = [None; W];
    let mut steps = Vec::new();
    for i in 0.. {
        for w in &mut workers {
            if let Some((k, t)) = w {
                *t -= 1;
                if *t == 0 {
                    steps.push(*k);
                    *w = None;
                }
            }
        }
        queue.extend(next(&steps, &queue, &input, &workers));
        queue.sort();
        for w in &mut workers {
            if w.is_none() && !queue.is_empty() {
                let next = queue.remove(0);
                *w = Some((next, next - 4));
            }
        }
        if workers.into_iter().all(|w| w.is_none()) {
            return (steps.into_iter().map(|c| c as char).collect(), i);
        }
    }
    unreachable!();
}

fn next(
    visited: &Vec<u8>,
    queue: &Vec<u8>,
    tree: &HashMap<u8, Vec<u8>>,
    workers: &[Option<(u8, u8)>],
) -> Vec<u8> {
    tree.into_iter()
        .filter_map(|(&k, rqs)| {
            (!visited.contains(&k)
                && !queue.contains(&k)
                && workers
                    .into_iter()
                    .filter_map(|&w| w)
                    .all(|(k2, _)| k2 != k)
                && rqs.into_iter().all(|r| visited.contains(r)))
            .then(|| k)
        })
        .collect()
}
