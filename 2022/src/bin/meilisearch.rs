advent_of_code_2022::main!();

fn generator(input: &'static str) -> HashMap<&'static str, Vec<Vec<bool>>> {
    #[derive(Clone, Debug)]
    struct Node(Option<&'static str>, Option<Box<Node>>, Option<Box<Node>>);

    let mut root = Node(None, None, None);
    for (name, path) in input.lines().map(|l| l.split_once(" - ").unwrap()) {
        let mut current = &mut root;
        for t in path.bytes() {
            current = match (t, current) {
                (b'L', Node(_, l, _)) => l,
                (b'R', Node(_, _, r)) => r,
                _ => unreachable!(),
            }
            .get_or_insert_with(|| Box::new(Node(None, None, None)));
        }
        current.0 = Some(name);
    }

    fn build_path(
        paths: &mut HashMap<&'static str, Vec<Vec<bool>>>,
        node: &Node,
        mut current: Vec<Vec<bool>>,
    ) {
        if let Node(Some(name), _, _) = node {
            paths.insert(name, current.clone());
        }
        match node {
            Node(None, Some(l), None) => {
                current.last_mut().unwrap().push(false);
                build_path(paths, l, current);
            }
            Node(None, None, Some(r)) => {
                current.last_mut().unwrap().push(true);
                build_path(paths, r, current);
            }
            Node(Some(_), Some(l), None) => {
                current.push(vec![false]);
                build_path(paths, l, current);
            }
            Node(Some(_), None, Some(r)) => {
                current.push(vec![true]);
                build_path(paths, r, current);
            }
            Node(_, Some(l), Some(r)) => {
                let mut current2 = current.clone();
                current2.push(vec![false]);
                build_path(paths, l, current2);
                current.push(vec![true]);
                build_path(paths, r, current);
            }
            _ => (),
        }
    }
    let mut paths = HashMap::new();
    build_path(&mut paths, &root, vec![]);

    paths
}

fn part_1(paths: HashMap<&'static str, Vec<Vec<bool>>>) -> &'static str {
    closest(&paths).0
}

fn part_2(mut paths: HashMap<&'static str, Vec<Vec<bool>>>) -> usize {
    let (mut current, mut score) = closest(&paths);
    while paths.len() > 1 {
        let (&next, distance) = paths
            .keys()
            .filter(|&&n| n != current)
            .map(|n| (n, distance(&paths, current, n)))
            .min_by(|&(n1, d1), &(n2, d2)| d1.cmp(&d2).then_with(|| paths[n1].cmp(&paths[n2])))
            .unwrap();
        paths.remove(current);
        current = next;
        score += distance;
    }
    score
}

fn closest<'a>(paths: &HashMap<&'a str, Vec<Vec<bool>>>) -> (&'a str, usize) {
    paths
        .iter()
        .sorted_by(|(_, p1), (_, p2)| p1.len().cmp(&p2.len()).then_with(|| p1.cmp(p2)))
        .next()
        .map(|(&n, p)| (n, p.len()))
        .unwrap()
}

fn distance(paths: &HashMap<&str, Vec<Vec<bool>>>, n1: &str, n2: &str) -> usize {
    let root = zip(&paths[n1], &paths[n2])
        .take_while(|(p1, p2)| p1 == p2)
        .count();
    paths[n1].len() + paths[n2].len() - root * 2
}
