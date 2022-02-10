advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<(i8, i8, i8, i8)> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(stars: Vec<(i8, i8, i8, i8)>) -> usize {
    let mut groups = stars
        .into_iter()
        .map(|s| vec![s])
        .enumerate()
        .collect::<HashMap<_, _>>();
    loop {
        if let Some((i, j)) = iproduct!(groups.iter(), groups.iter())
            .filter(|((i, _), (j, _))| i != j)
            .find(|((_, g1), (_, g2))| {
                iproduct!(g1.iter(), g2.iter())
                    .any(|(&(a, b, c, d), &(w, x, y, z))| m_dist!(a, b, c, d; w, x, y, z) <= 3)
            })
            .map(|((i, _), (j, _))| (*i, *j))
        {
            let to_move = groups.remove(&j).unwrap();
            groups.get_mut(&i).unwrap().extend(to_move);
        } else {
            return groups.len();
        }
    }
}

fn part_2(_: Vec<(i8, i8, i8, i8)>) -> &'static str {
    "N/A"
}
