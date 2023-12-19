advent_of_code_2023::main!();

#[derive(Copy, Clone, Debug)]
enum Rule<'a> {
    Pass,
    Cmp(&'a str, u64, fn(&u64, &u64) -> bool),
}

fn generator(input: &str) -> (HashMap<&str, Vec<(Rule, &str)>>, Vec<HashMap<&str, u64>>) {
    let (wfs, parts) = input.split_once("\n\n").unwrap();
    (
        wfs.lines()
            .map(|l| {
                let (key, rules) = l.trim_end_matches('}').split_once('{').unwrap();
                (
                    key,
                    rules
                        .split(',')
                        .map(|r| match r.split(['<', '>', ':']).collect_tuple() {
                            Some((v, n, d)) => (
                                Rule::Cmp(
                                    v,
                                    n.parse().unwrap(),
                                    if r.contains('<') { u64::lt } else { u64::gt },
                                ),
                                d,
                            ),
                            None => (Rule::Pass, r),
                        })
                        .collect(),
                )
            })
            .collect(),
        parts
            .lines()
            .map(|l| {
                l.trim_matches(&['{', '}'][..])
                    .split(',')
                    .map(|v| {
                        let (k, v) = v.split_once('=').unwrap();
                        (k, v.parse().unwrap())
                    })
                    .collect()
            })
            .collect(),
    )
}

fn part_1((wfs, parts): (HashMap<&str, Vec<(Rule, &str)>>, Vec<HashMap<&str, u64>>)) -> u64 {
    parts
        .into_iter()
        .map(|p| {
            let mut wf = "in";
            loop {
                wf = wfs[wf]
                    .iter()
                    .find_map(|&(r, d)| match r {
                        Rule::Pass => Some(d),
                        Rule::Cmp(k, c, f) => f(&p[k], &c).then_some(d),
                    })
                    .unwrap();
                match wf {
                    "A" => return p.values().sum(),
                    "R" => return 0,
                    _ => continue,
                }
            }
        })
        .sum()
}

fn part_2((wfs, _): (HashMap<&str, Vec<(Rule, &str)>>, Vec<HashMap<&str, u64>>)) -> u64 {
    possibilities(&wfs, "in", [[1, 4_000]; 4])
}

fn possibilities(
    wfs: &HashMap<&str, Vec<(Rule, &str)>>,
    wf: &str,
    mut parts: [[u64; 2]; 4],
) -> u64 {
    match wf {
        "A" => return parts.into_iter().map(|[l, u]| u - l + 1).product(),
        "R" => return 0,
        _ => (),
    }
    wfs[wf]
        .iter()
        .map(|&(r, d)| match r {
            Rule::Pass => possibilities(&wfs, d, parts),
            Rule::Cmp(k, c, f) => {
                let i = b"xmas".iter().position(|&p| k.as_bytes()[0] == p).unwrap();
                let lt = (f as usize == u64::lt as usize) as usize;
                let mut dup = parts;
                dup[i][lt] = c + 1 - (lt << 1) as u64;
                parts[i][lt ^ 1] = c;
                possibilities(&wfs, d, dup)
            }
        })
        .sum()
}
