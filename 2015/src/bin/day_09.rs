advent_of_code_2015::main!();

fn generator(input: &str) -> HashMap<String, HashMap<String, u64>> {
    let mut map = HashMap::new();
    input
        .lines()
        .map(|l| l.split(' ').collect_vec())
        .map(|p| {
            (
                p[0].to_string(),
                p[2].to_string(),
                p[4].parse::<u64>().unwrap(),
            )
        })
        .for_each(|(from, to, dist)| {
            map.entry(from.clone())
                .or_insert(HashMap::new())
                .insert(to.clone(), dist);
            map.entry(to).or_insert(HashMap::new()).insert(from, dist);
        });

    map
}

fn part_1(input: HashMap<String, HashMap<String, u64>>) -> u64 {
    solver(&input).min().unwrap()
}

fn part_2(input: HashMap<String, HashMap<String, u64>>) -> u64 {
    solver(&input).max().unwrap()
}

fn solver<'a>(input: &'a HashMap<String, HashMap<String, u64>>) -> impl Iterator<Item = u64> + 'a {
    input.keys().permutations(input.len()).map(move |cs| {
        cs.iter()
            .tuple_windows()
            .map(|(c1, c2)| input.get(c1.clone()).unwrap().get(c2.clone()).unwrap())
            .sum()
    })
}
