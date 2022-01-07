advent_of_code_2017::main!();

fn generator(input: &str) -> HashMap<&str, (u32, Vec<&str>)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split([' ', '(', ')', ','].as_slice()).collect_vec();
            (
                parts[0],
                (
                    parts[2].parse().unwrap(),
                    parts.into_iter().skip(5).step_by(2).collect(),
                ),
            )
        })
        .collect()
}

fn part_1<'a>(input: HashMap<&'a str, (u32, Vec<&str>)>) -> &'a str {
    bottom(&input)
}

fn part_2(input: HashMap<&str, (u32, Vec<&str>)>) -> u32 {
    unbalanced(&input, bottom(&input))
}

fn bottom<'a>(tree: &HashMap<&'a str, (u32, Vec<&str>)>) -> &'a str {
    let mut current = tree.keys().next().unwrap();
    loop {
        if let Some(next) = tree
            .iter()
            .find_map(|(from, (_, to))| to.contains(current).then(|| from))
        {
            current = next;
        } else {
            return current;
        }
    }
}

fn unbalanced<'a>(tree: &HashMap<&'a str, (u32, Vec<&'a str>)>, disc: &'a str) -> u32 {
    let ws = tree[disc].1.iter().fold(HashMap::new(), |mut acc, &disc| {
        let sum = sum(tree, disc);
        acc.entry(sum).or_insert(Vec::new()).push(disc);
        acc
    });
    let (ub_w, ub) = ws
        .iter()
        .find(|(_w, sd)| sd.len() == 1)
        .map(|(&w, sd)| (w, sd[0]))
        .unwrap();
    if tree[ub].1.iter().map(|d| sum(tree, d)).all_equal() {
        tree[ub].0 - (ub_w - ws.iter().find(|(_w, sd)| sd.len() > 1).unwrap().0)
    } else {
        unbalanced(tree, ub)
    }
}

fn sum(tree: &HashMap<&str, (u32, Vec<&str>)>, disc: &str) -> u32 {
    let (w, to) = &tree[disc];
    w + to.into_iter().map(|d| sum(tree, d)).sum::<u32>()
}
