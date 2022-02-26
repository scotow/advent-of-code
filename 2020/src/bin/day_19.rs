advent_of_code_2020::main!();

type SubRule = Vec<u8>;

#[derive(Debug, Clone)]
enum Rule {
    Terminal(u8),
    Recursive(SubRule, Option<SubRule>),
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('"') {
            Rule::Terminal(s.as_bytes()[1])
        } else {
            let parts = s.split(" | ").collect_vec();
            let right = parse_sub_rule(parts[0]);
            match parts.len() {
                1 => Rule::Recursive(right, None),
                2 => Rule::Recursive(right, Some(parse_sub_rule(parts[1]))),
                _ => unreachable!(),
            }
        })
    }
}

impl Rule {
    fn resolve(&self, map: &HashMap<u8, Rule>) -> Vec<Vec<u8>> {
        match self {
            Rule::Terminal(c) => vec![vec![*c]],
            Rule::Recursive(r1, p2) => {
                let mut res = Rule::resolve_sub_rule(&map, r1);
                if let Some(p2) = p2 {
                    res.append(&mut Rule::resolve_sub_rule(&map, p2))
                }
                res
            }
        }
    }

    fn resolve_sub_rule(map: &HashMap<u8, Rule>, r: &SubRule) -> Vec<Vec<u8>> {
        let mut res = map[&r[0]].resolve(&map);
        for k in r.iter().skip(1) {
            let act = map[k].resolve(&map);
            res = res
                .into_iter()
                .cartesian_product(act)
                .map(|(x, y)| [x, y].concat())
                .collect()
        }
        res
    }
}

fn parse_sub_rule(s: &str) -> SubRule {
    s.split(' ').map(|n| n.parse().unwrap()).collect()
}

fn generator(input: &str) -> (HashMap<u8, Rule>, Vec<Vec<u8>>) {
    let (rules, messages) = input.split("\n\n").collect_tuple().unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (id, rule) = l.split(": ").collect_tuple().unwrap();
            (id.parse().unwrap(), rule.parse().unwrap())
        })
        .collect();
    let messages = messages.lines().map(|l| l.as_bytes().to_vec()).collect();

    (rules, messages)
}

fn has_prefix(m: &[u8], r1: &[Vec<u8>], r2: &[Vec<u8>]) -> bool {
    r1.iter()
        .filter(|&r| m.starts_with(r))
        .map(|r| &m[r.len()..])
        .any(|m| has_prefix(m, r1, r2) || has_middle(m, r1, r2))
}

fn has_middle(m: &[u8], r1: &[Vec<u8>], r2: &[Vec<u8>]) -> bool {
    r1.iter()
        .cartesian_product(r2.iter())
        .filter(|&(x, y)| m.starts_with(x) && m.ends_with(y) && x.len() + y.len() <= m.len())
        .map(|(x, y)| &m[x.len()..(m.len() - y.len())])
        .any(|m| m.len() == 0 || has_middle(m, r1, r2))
}

fn part_1(input: (HashMap<u8, Rule>, Vec<Vec<u8>>)) -> usize {
    let possibilities = input.0[&0].resolve(&input.0);
    input
        .1
        .iter()
        .filter(|&m| possibilities.contains(m))
        .count()
}

fn part_2(input: (HashMap<u8, Rule>, Vec<Vec<u8>>)) -> usize {
    input
        .1
        .iter()
        .filter(|m| {
            has_prefix(
                m,
                &input.0[&42].resolve(&input.0),
                &input.0[&31].resolve(&input.0),
            )
        })
        .count()
}
