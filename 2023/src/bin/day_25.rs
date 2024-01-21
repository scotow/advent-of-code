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

fn part_1(input: HashMap<&str, Vec<&str>>) -> usize {
    let g = input
        .iter()
        .flat_map(|(k, vs)| vs.iter().map(move |v| (k, v)))
        .map(|(k, v)| format!("{} -> {};", k, v))
        .join("\n");
    println!("{g}");

    let to_remove = input
        .keys()
        .tuple_combinations()
        .filter(|&(l, r)| input[l].contains(r))
        .tuple_combinations()
        .find(|&((a1, a2), (b1, b2), (c1, c2))| {
            let mut map = input.clone();
            remove_links(&mut map, [(*a1, *a2), (*b1, *b2), (*c1, *c2)]);
            size(&map, map.keys().next().unwrap()) != input.len()
        })
        .unwrap();
    dbg!(to_remove);

    let mut links = input.clone();
    remove_links(
        &mut links,
        [
            (*to_remove.0 .0, *to_remove.0 .1),
            (*to_remove.1 .0, *to_remove.1 .1),
            (*to_remove.2 .0, *to_remove.2 .1),
        ],
    );
    links.keys().map(|k| size(&links, k)).unique().product()
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
