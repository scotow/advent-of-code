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
    solve([b'A'; 3], |n| n == [b'Z'; 3], &steps, &map)
}

fn part_2((steps, map): (Vec<usize>, HashMap<Node, [Node; 2]>)) -> usize {
    map.keys()
        .filter(|n| n[2] == b'A')
        .map(|&n| solve(n, |n| n[2] == b'Z', &steps, &map))
        .reduce(|a, n| a.lcm(&n))
        .unwrap()
}

fn solve<F: Fn(Node) -> bool>(
    mut node: Node,
    finish: F,
    steps: &[usize],
    map: &HashMap<Node, [Node; 2]>,
) -> usize {
    1 + steps
        .into_iter()
        .cycle()
        .position(|&s| {
            node = map[&node][s];
            finish(node)
        })
        .unwrap()
}
