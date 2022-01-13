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
    dbg!(&input);
    solve(input, 0)
}

fn part_2(input: HashSet<(u16, u16)>) -> u16 {
    solve2(input, 0, 0).0
}

fn solve(links: HashSet<(u16, u16)>, current: u16) -> u16 {
    // dbg!(current);
    current
        + links
            .iter()
            .filter(|&&(a, b)| a == current || b == current)
            .map(|&(a, b)| {
                let mut new = links.clone();
                new.remove(&(a, b));
                if current == a {
                    a + solve(new, b)
                } else {
                    b + solve(new, a)
                }
            })
            .max()
            .unwrap_or(0)
}

fn solve2(links: HashSet<(u16, u16)>, current: u16, depth: usize) -> (u16, usize) {
    // dbg!(current);
    current
        + links
            .iter()
            .filter(|&&(a, b)| a == current || b == current)
            .map(|&(a, b)| {
                let mut new = links.clone();
                new.remove(&(a, b));
                if current == a {
                    let (strength, depth) = solve2(new, b, depth + 1)
                    (a + solve2(new, b), depth + 1)
                } else {
                    (b + solve2(new, a), depth + 1)
                }
            })
            .max_by(|(s1, d1), (s2, d2)| d1.cmp(d2).then(s1.cmp(s2)))
            .unwrap_or((0, 1))
}
