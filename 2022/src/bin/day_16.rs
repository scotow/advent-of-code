advent_of_code_2022::main!();

type Valves<'a> = HashMap<&'a str, (u32, Vec<&'a str>)>;

fn generator(input: &str) -> Valves {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split([' ', '=', ';', ',']);
            (
                parts.nth(1).unwrap(),
                (
                    parts.nth(3).unwrap().parse().unwrap(),
                    parts.skip(5).step_by(2).collect(),
                ),
            )
        })
        .collect()
}

fn part_1(valves: Valves) -> u32 {
    let worthy_valves = worthy_valves(&valves);
    let dists = dists_map(&valves, &worthy_valves);
    solve::<30>(&valves, &dists, "AA", 1, 0, 0, worthy_valves)
}

fn part_2(valves: Valves) -> u32 {
    let worthy_valves = worthy_valves(&valves);
    let dists = dists_map(&valves, &worthy_valves);
    worthy_valves
        .iter()
        .copied()
        .powerset()
        .map(|set| {
            let human = set.iter().copied().collect();
            let elephant = worthy_valves.difference(&human).copied().collect();
            solve::<26>(&valves, &dists, "AA", 1, 0, 0, human)
                + solve::<26>(&valves, &dists, "AA", 1, 0, 0, elephant)
        })
        .max()
        .unwrap()
}

fn worthy_valves<'a>(valves: &Valves<'a>) -> HashSet<&'a str> {
    valves
        .iter()
        .filter(|(_, &(bw, _))| bw > 0)
        .map(|(&n, _)| n)
        .collect()
}

fn dists_map<'a>(
    valves: &Valves<'a>,
    worthy: &HashSet<&'a str>,
) -> HashMap<&'a str, HashMap<&'a str, u32>> {
    chain!(once(&"AA"), worthy.iter())
        .map(|name| {
            (
                *name,
                dijkstra_all(name, |n| valves[n].1.iter().map(|&n| (n, 1)))
                    .into_iter()
                    .filter(|(n, _)| worthy.contains(n))
                    .map(|(n, (_, c))| (n, c))
                    .collect(),
            )
        })
        .collect()
}

fn solve<const MAX_MIN: u32>(
    valves: &Valves,
    dists: &HashMap<&str, HashMap<&str, u32>>,
    current: &str,
    min: u32,
    pps: u32,
    released: u32,
    to_visit: HashSet<&str>,
) -> u32 {
    if to_visit.is_empty() {
        return released + (MAX_MIN - min + 1) * pps;
    }
    to_visit
        .iter()
        .map(|&next| {
            let mut to_visit = to_visit.clone();
            to_visit.remove(next);
            if min + dists[current][next] >= MAX_MIN {
                return released + (MAX_MIN - min + 1) * pps;
            }
            solve::<MAX_MIN>(
                valves,
                dists,
                next,
                min + dists[current][next] + 1,
                pps + valves[next].0,
                released + pps * (dists[current][next] + 1),
                to_visit,
            )
        })
        .max()
        .unwrap()
}
