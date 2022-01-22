advent_of_code_2018::main!();

fn generator(input: &str) -> HashMap<u32, Vec<Range<u32>>> {
    let mut guards = HashMap::new();
    let mut lines = input.lines().sorted().peekable();
    while let Some(l) = lines.next() {
        let guard = guards
            .entry(l.split(['#', ' ']).nth(4).unwrap().parse().unwrap())
            .or_insert(Vec::new());
        while lines.peek().map(|l| !l.contains("Guard")).unwrap_or(false) {
            let (s, e) = lines
                .by_ref()
                .take(2)
                .map(|l| l.split([':', ']']).nth(1).unwrap().parse().unwrap())
                .collect_tuple()
                .unwrap();
            guard.push(s..e);
        }
    }
    guards
}

fn part_1(input: HashMap<u32, Vec<Range<u32>>>) -> u32 {
    let (id, ranges) = input
        .into_iter()
        .max_by_key(|(_, ss)| ss.into_iter().map(|s| s.len()).sum::<usize>())
        .unwrap();
    id * ranges
        .into_iter()
        .flatten()
        .counts()
        .into_iter()
        .max_by_key(|&(_, n)| n)
        .unwrap()
        .0
}

fn part_2(input: HashMap<u32, Vec<Range<u32>>>) -> u32 {
    let (id, (m, _)) = input
        .into_iter()
        .filter(|(_, ss)| !ss.is_empty())
        .map(|(id, ss)| {
            (
                id,
                ss.into_iter()
                    .flatten()
                    .counts()
                    .into_iter()
                    .max_by_key(|&(_, n)| n)
                    .unwrap(),
            )
        })
        .max_by_key(|&(_, (_, n))| n)
        .unwrap();
    id * m
}
