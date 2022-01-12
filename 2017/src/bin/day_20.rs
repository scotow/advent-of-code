advent_of_code_2017::main!();

type Coord = (i32, i32, i32);

fn generator(input: &str) -> Vec<(Coord, Coord, Coord)> {
    input
        .lines()
        .map(|l| {
            let (a, b, c, d, e, f, g, h, i) = l
                .split(['<', '>', ','].as_slice())
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .unwrap();
            ((a, b, c), (d, e, f), (g, h, i))
        })
        .collect()
}

fn part_1(input: Vec<(Coord, Coord, Coord)>) -> usize {
    input
        .into_iter()
        .enumerate()
        .min_by(|(_, (_, v1, a1)), (_, (_, v2, a2))| {
            (a1.0.abs() + a1.1.abs() + a1.2.abs())
                .cmp(&(a2.0.abs() + a2.1.abs() + a2.2.abs()))
                .then(
                    (v1.0.abs() + v1.1.abs() + v1.2.abs())
                        .cmp(&(v2.0.abs() + v2.1.abs() + v2.2.abs())),
                )
        })
        .unwrap()
        .0
}

fn part_2(input: Vec<(Coord, Coord, Coord)>) -> usize {
    let mut state = input.into_iter().enumerate().fold(
        HashMap::<_, Vec<_>>::new(),
        |mut acc, (i, (p, v, a))| {
            acc.entry(p).or_default().push((i, v, a));
            acc
        },
    );
    let mut order = positions(&state);
    loop {
        state = state
            .into_iter()
            .flat_map(|(p, pixels)| {
                pixels.into_iter().map(move |(i, mut v, a)| {
                    v.0 += a.0;
                    v.1 += a.1;
                    v.2 += a.2;
                    (i, (p.0 + v.0, p.1 + v.1, p.2 + v.2), v, a)
                })
            })
            .fold(HashMap::new(), |mut acc, (i, p, v, a)| {
                acc.entry(p).or_default().push((i, v, a));
                acc
            });
        state.retain(|_k, v| v.len() == 1);
        let new_order = positions(&state);
        if new_order == order {
            return state.len();
        } else {
            order = new_order;
        }
    }
}

fn positions(pixels: &HashMap<Coord, Vec<(usize, Coord, Coord)>>) -> Vec<usize> {
    pixels
        .into_iter()
        .flat_map(|(pos, pixels)| pixels.into_iter().map(move |(i, _, _)| (pos, i)))
        .sorted_by_key(|(pos, _)| pos.0.abs() + pos.1.abs() + pos.2.abs())
        .map(|(_, &i)| i)
        .collect()
}
