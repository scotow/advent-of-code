advent_of_code_2017::main!();

fn generator(input: &str) -> HashMap<u16, Vec<u16>> {
    input
        .lines()
        .map(|l| {
            let (from, to) = l.split(" <-> ").collect_tuple().unwrap();
            (
                from.parse().unwrap(),
                to.split(", ").map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part_1(input: HashMap<u16, Vec<u16>>) -> usize {
    let mut visited = HashSet::new();
    visit(&input, &mut visited, 0);
    visited.len()
}

fn part_2(input: HashMap<u16, Vec<u16>>) -> usize {
    let mut groups = 0;
    let mut all_visited = HashSet::new();
    for prog in 0..=*input.keys().max().unwrap() {
        if all_visited.contains(&prog) {
            continue;
        }
        let mut visited = HashSet::new();
        visit(&input, &mut visited, prog);
        groups += 1;
        all_visited.extend(visited);
    }
    groups
}

fn visit(mapping: &HashMap<u16, Vec<u16>>, visited: &mut HashSet<u16>, prog: u16) {
    if visited.contains(&prog) {
        return;
    }
    visited.insert(prog);
    for &connection in &mapping[&prog] {
        visit(mapping, visited, connection);
    }
}
