advent_of_code_2023::main!();

fn generator(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut links: HashMap<_, Vec<_>> = HashMap::new();
    for (l, r) in input.lines().flat_map(|l| {
        let (l, r) = l.split_once(": ").unwrap();
        r.split(' ').flat_map(move |r| [(l, r), (r, l)])
    }) {
        links.entry(l).or_default().push(r)
    }
    links
}

fn part_1(mut input: HashMap<&str, Vec<&str>>) -> usize {
    let to_remove = input
        .keys()
        .flat_map(|start| {
            let all = dijkstra_all(start, |n| input[n].iter().map(|&t| (t, 1)));
            all.keys()
                .flat_map(|t| dijkstra_path(t, &all))
                .collect::<Vec<_>>()
        })
        .counts()
        .into_iter()
        .sorted_by_key(|&(_, n)| usize::MAX - n)
        .take(6)
        .map(|(k, _)| k)
        .permutations(6)
        .find(|cs| cs.chunks(2).all(|ts| input[ts[0]].contains(&ts[1])))
        .unwrap()
        .into_iter()
        .tuples()
        .collect::<Vec<_>>();
    remove_links(&mut input, to_remove);
    input
        .keys()
        .map(|k| size(&input, k))
        .dedup()
        .take(2)
        .product()
}

fn remove_links<'a>(
    links: &mut HashMap<&str, Vec<&str>>,
    ls: impl IntoIterator<Item = (&'a str, &'a str)>,
) {
    for (l1, l2) in ls {
        let index = links[l1].iter().position(|&t| l2 == t).unwrap();
        links.get_mut(l1).unwrap().remove(index);
        let index = links[l2].iter().position(|&t| l1 == t).unwrap();
        links.get_mut(l2).unwrap().remove(index);
    }
}

fn size(links: &HashMap<&str, Vec<&str>>, start: &str) -> usize {
    let mut to_visit = vec![start];
    let mut visited = HashSet::new();
    while !to_visit.is_empty() {
        let next = to_visit.pop().unwrap();
        visited.insert(next);
        for t in &links[next] {
            if visited.contains(t) {
                continue;
            }
            to_visit.push(t);
        }
    }
    visited.len()
}

fn part_2(_: HashMap<&str, Vec<&str>>) -> &'static str {
    "N/A"
}
