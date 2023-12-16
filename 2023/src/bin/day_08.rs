advent_of_code_2023::main!();

type Node = [u8; 3];

fn generator(input: &str) -> (Vec<usize>, HashMap<Node, [Node; 2]>) {
    let (steps, map) = input.split_once("\n\n").unwrap();
    (
        steps.bytes().map(|b| (b == b'R') as usize).collect(),
        map.lines()
            .map(|l| {
                let (f, l, r) = l
                    .bytes()
                    .filter(u8::is_ascii_alphanumeric)
                    .chunks(3)
                    .into_iter()
                    .map(|c| c.collect::<Vec<_>>().try_into().unwrap())
                    .collect_tuple()
                    .unwrap();
                (f, [l, r])
            })
            .collect(),
    )
}

fn part_1((steps, map): (Vec<usize>, HashMap<Node, [Node; 2]>)) -> usize {
    solve([b'A'; 3], &[b'Z'; 3], &steps, &map)
}

fn part_2((steps, map): (Vec<usize>, HashMap<Node, [Node; 2]>)) -> usize {
    map.keys()
        .filter(|n| n[2] == b'A')
        .map(|&n| solve(n, &[b'Z'], &steps, &map))
        .reduce(|a, n| a.lcm(&n))
        .unwrap()
}

fn solve(mut node: Node, end: &[u8], steps: &[usize], map: &HashMap<Node, [Node; 2]>) -> usize {
    1 + steps
        .iter()
        .cycle()
        .position(|&s| {
            node = map[&node][s];
            node.ends_with(end)
        })
        .unwrap()
}
