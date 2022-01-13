advent_of_code_2017::main!();

fn generator(input: &str) -> HashSet<(u16, u16)> {
    input
        .lines()
        .map(|l| {
            l.split('/')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: HashSet<(u16, u16)>) -> u16 {
    solve(input, 0, false).0
}

fn part_2(input: HashSet<(u16, u16)>) -> u16 {
    solve(input, 0, true).0
}

fn solve(links: HashSet<(u16, u16)>, current: u16, wants_depth: bool) -> (u16, usize) {
    links
        .iter()
        .filter(|&&(a, b)| a == current || b == current)
        .map(|&(a, b)| {
            let mut new = links.clone();
            new.remove(&(a, b));
            let (strength, depth) = solve(new, if current == a { b } else { a }, wants_depth);
            (a + b + strength, depth + 1)
        })
        .max_by(|(s1, d1), (s2, d2)| {
            if wants_depth {
                d1.cmp(d2).then(s1.cmp(s2))
            } else {
                s1.cmp(s2)
            }
        })
        .unwrap_or((0, 1))
}
