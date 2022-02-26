advent_of_code_2015::main!();

fn generator(input: &str) -> HashMap<String, HashMap<String, i64>> {
    let mut map = HashMap::new();
    input
        .lines()
        .map(|l| l[..l.len() - 1].split(' ').collect_vec())
        .map(|p| {
            (
                p[0].to_string(),
                p[10].to_string(),
                p[3].parse::<i64>().unwrap() * if p[2] == "gain" { 1 } else { -1 },
            )
        })
        .for_each(|(from, to, score)| {
            map.entry(from).or_insert(HashMap::new()).insert(to, score);
        });
    map
}

fn part_1(input: HashMap<String, HashMap<String, i64>>) -> i64 {
    solve(input)
}

fn part_2(input: HashMap<String, HashMap<String, i64>>) -> i64 {
    let mut input = input.clone();
    input.insert("Me".into(), HashMap::new());
    let people = input.keys().cloned().collect_vec();
    for p in people {
        input.get_mut(&p).unwrap().insert("Me".into(), 0);
        input.get_mut("Me").unwrap().insert(p, 0);
    }

    solve(input)
}

fn solve(input: HashMap<String, HashMap<String, i64>>) -> i64 {
    input
        .keys()
        .permutations(input.len())
        .map(|t| {
            t.iter()
                .chain(once(t.first().unwrap()))
                .tuple_windows()
                .map(|(&l, &r)| input[l][r] + input[r][l])
                .sum()
        })
        .max()
        .unwrap()
}
