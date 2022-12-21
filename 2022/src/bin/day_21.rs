advent_of_code_2022::main!();

type Map<'a> = HashMap<&'a str, Either<i64, (&'a str, &'a str, fn(i64, i64) -> i64)>>;

fn generator(input: &str) -> Map {
    input
        .lines()
        .map(|l| {
            (
                &l[0..4],
                l[6..].parse().map(|n| Either::Left(n)).unwrap_or_else(|_| {
                    Either::Right((
                        &l[6..10],
                        &l[13..17],
                        match l.as_bytes()[11] {
                            b'+' => i64::add,
                            b'-' => i64::sub,
                            b'*' => i64::mul,
                            b'/' => i64::div,
                            _ => unreachable!(),
                        },
                    ))
                }),
            )
        })
        .collect()
}

fn part_1(input: Map) -> i64 {
    resolve(&input, "root")
}

fn part_2(mut input: Map) -> i64 {
    let other = *input
        .iter()
        .find_map(|(_, v)| match v {
            Either::Right(("humn", o, _)) => Some(o),
            Either::Right((o, "humn", _)) => Some(o),
            _ => None,
        })
        .unwrap();
    dbg!(other);
    dbg!(resolve(&input, other));

    // let (k1, k2, _) = input["root"].unwrap_right();
    // for i in 1.. {
    //     *input.get_mut("humn").unwrap() = Either::Left(i);
    //     if resolve(&input, k1) == resolve(&input, k2) {
    //         return i;
    //     }
    // }
    // unreachable!()

    0
}

fn resolve(mapping: &Map, key: &str) -> i64 {
    match mapping[key] {
        Either::Left(n) => n,
        Either::Right((k1, k2, op)) => op(resolve(mapping, k1), resolve(mapping, k2)),
    }
}
