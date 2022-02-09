advent_of_code_2018::main!();

#[derive(Clone, Debug)]
struct Group {
    units: usize,
    hp: usize,
    ap: usize,
    attack: &'static str,
    init: usize,
    weak: Vec<&'static str>,
    immune: Vec<&'static str>,
}

impl Group {
    fn parse(s: &'static str) -> Self {
        let parts = s.split_whitespace().collect_vec();
        let mut weak = Vec::new();
        let mut immune = Vec::new();
        let mut shift = 0;
        if s.contains('(') {
            let specs = s.trim_matches(|c| !['(', ')'].contains(&c));
            let specs = &specs[1..specs.len() - 1];
            for s in specs.split("; ") {
                let (to, _, kinds) = s.splitn(3, ' ').collect_tuple().unwrap();
                let kinds = kinds.split(", ").collect_vec();
                *match to {
                    "weak" => &mut weak,
                    "immune" => &mut immune,
                    _ => unreachable!(),
                } = kinds;
            }
            shift = specs.split_whitespace().count();
        }
        Group {
            units: parts[0].parse().unwrap(),
            hp: parts[4].parse().unwrap(),
            ap: parts[12 + shift].parse().unwrap(),
            attack: parts[13 + shift],
            init: parts[17 + shift].parse().unwrap(),
            weak,
            immune,
        }
    }
}

fn generator(input: &'static str) -> (Vec<Group>, Vec<Group>) {
    input
        .split("\n\n")
        .map(|side| side.lines().skip(1).map(|l| Group::parse(l)).collect())
        .collect_tuple()
        .unwrap()
}

fn part_1(input: (Vec<Group>, Vec<Group>)) -> u8 {
    0
}

fn part_2(input: (Vec<Group>, Vec<Group>)) -> u8 {
    0
}
