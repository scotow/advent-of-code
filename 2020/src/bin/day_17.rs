advent_of_code_2020::main!();

type Position = (i16, i16, i16, i16);

fn generator(input: &str) -> HashSet<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i16, y as i16, 0, 0))
        })
        .collect()
}

fn part_1(input: HashSet<Position>) -> usize {
    solve(&input, 0..=0)
}

fn part_2(input: HashSet<Position>) -> usize {
    solve(&input, -1..=1)
}

fn solve(map: &HashSet<Position>, w: RangeInclusive<i16>) -> usize {
    let mut current = map.clone();
    for _ in 0..6 {
        let mut next = HashSet::new();
        for &active in &current {
            if (2..=3).contains(&neighbors(&current, active, &w)) {
                next.insert(active);
            }
            for inactive in adjacent(active, &w) {
                if neighbors(&current, inactive, &w) == 3 {
                    next.insert(inactive);
                }
            }
        }
        current = next;
    }
    current.len()
}

fn adjacent(pos: Position, w: &RangeInclusive<i16>) -> impl Iterator<Item = Position> {
    iproduct!(-1..=1, -1..=1, -1..=1, w.clone())
        .filter(|&p| p != (0, 0, 0, 0))
        .map(move |(x, y, z, w)| (pos.0 + x, pos.1 + y, pos.2 + z, pos.3 + w))
}

fn neighbors(map: &HashSet<Position>, pos: Position, w: &RangeInclusive<i16>) -> usize {
    adjacent(pos, w).filter(|p| map.contains(p)).count()
}
