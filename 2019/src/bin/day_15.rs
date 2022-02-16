use pathfinding::prelude::dijkstra_all;

advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> usize {
    let res = solve(prog).0;
    assert_eq!(res, 234);
    res
}

fn part_2(prog: Program) -> usize {
    let (_, (ox, oy), grid) = solve(prog);
    let mut queue = HashSet::from([(ox, oy)]);
    let mut visited = HashSet::new();
    let mut n = 0;
    loop {
        visited.extend(queue.iter().copied());
        let next = queue
            .into_iter()
            .flat_map(|(x, y)| [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)])
            .filter(|xy| grid.contains(xy) && !visited.contains(xy))
            .collect::<HashSet<_>>();
        if next.is_empty() {
            assert_eq!(n, 292);
            return n;
        }
        queue = next;
        n += 1;
    }
}

fn solve(prog: Program) -> (usize, (isize, isize), HashSet<(isize, isize)>) {
    let mut states = HashMap::new();
    states.insert((0, 0), prog.clone());
    let mut oxygen = (0, 0);
    let paths = dijkstra_all(&(0, 0), |&(x, y)| {
        let prog = states[&(x, y)].clone();
        let mut next = Vec::with_capacity(4);
        for (i, dx, dy) in [(1, 0, -1), (2, 0, 1), (3, -1, 0), (4, 1, 0)] {
            let (x, y) = (x + dx, y + dy);
            let mut prog = prog.clone();
            prog.push(i);
            prog.run();
            let status = prog.pull().unwrap();
            if status >= 1 {
                states.insert((x, y), prog);
                next.push(((x, y), 1));
            }
            if status == 2 {
                oxygen = (x, y);
            }
        }
        next
    });
    (paths[&oxygen].1, oxygen, states.into_keys().collect())
}
