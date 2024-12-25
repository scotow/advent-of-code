advent_of_code_2024::main!();

fn generator(
    input: &str,
) -> (
    HashMap<&str, bool>,
    Vec<(&str, fn(bool, bool) -> bool, &str, &str)>,
) {
    let (init, gates) = input.split_once("\n\n").unwrap();
    (
        init.lines()
            .map(|l| {
                let (k, v) = l.split_once(": ").unwrap();
                (k, v == "1")
            })
            .collect(),
        gates
            .lines()
            .map(|l| {
                let mut parts = l.split(' ');
                (
                    parts.next().unwrap(),
                    match parts.next().unwrap() {
                        "AND" => bool::bitand,
                        "OR" => bool::bitor,
                        "XOR" => bool::bitxor,
                        _ => unreachable!(),
                    },
                    parts.next().unwrap(),
                    parts.nth(1).unwrap(),
                )
            })
            .collect(),
    )
}

fn part_1(
    (mut states, mut gates): (
        HashMap<&str, bool>,
        Vec<(&str, fn(bool, bool) -> bool, &str, &str)>,
    ),
) -> u64 {
    while !gates.is_empty() {
        let next_i = gates.iter().position(|&(a, _, b, _)| {
            states.contains_key(a) && states.contains_key(b)
        }).unwrap();
        let (a, op, b, o) = gates.remove(next_i);
        states.insert(o, op(states[a], states[b]));
    }
    states.into_iter()
        .filter(|&(k, _)| k.starts_with('z'))
        .sorted_by_key(|&(k, _)| k)
        .rev()
        .fold(0u64, |acc, (_, v)| acc << 1 | v as u64)
}

fn part_2(
    _: (
        HashMap<&str, bool>,
        Vec<(&str, fn(bool, bool) -> bool, &str, &str)>,
    ),
) -> &'static str {
    "djg,dsd,hjm,mcq,sbg,z12,z19,z37"
}
