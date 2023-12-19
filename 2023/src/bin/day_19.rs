advent_of_code_2023::main!();

fn pass(_: u64, _: u64) -> bool {
    true
}

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
                            Some((v, n, d)) => {
                                // dbg!(v, r, d);
                                let n = n.parse().unwrap();
                                let c = if r.contains('<') { u64::lt } else { u64::gt };
                                (Rule::Cmp(v, n, c), d)
                            }
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
    // dbg!(&wfs, &parts);
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
    possibilities(&wfs, "in", (1, 4_000), (1, 4_000), (1, 4_000), (1, 4_000))
}

fn possibilities(
    wfs: &HashMap<&str, Vec<(Rule, &str)>>,
    wf: &str,
    mut x: (u64, u64),
    mut m: (u64, u64),
    mut a: (u64, u64),
    mut s: (u64, u64),
) -> u64 {
    match wf {
        "A" => return (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1),
        "R" => return 0,
        _ => (),
    };
    wfs[wf]
        .iter()
        .map(|&(r, d)| {
            dbg!(r, d, x, m, a, s);
            match r {
                Rule::Pass => possibilities(&wfs, d, x, m, a, s),
                Rule::Cmp(k, c, f) => {
                    let lt = f as usize == u64::lt as usize;
                    match (k, lt) {
                        ("x", true) => {
                            let r = possibilities(&wfs, d, (x.0, c - 1), m, a, s);
                            x.0 = c;
                            r
                        }
                        ("x", false) => {
                            let r = possibilities(&wfs, d, (c + 1, x.1), m, a, s);
                            x.1 = c;
                            r
                        }
                        ("m", true) => {
                            let r = possibilities(&wfs, d, x, (m.0, c - 1), a, s);
                            m.0 = c;
                            r
                        }
                        ("m", false) => {
                            let r = possibilities(&wfs, d, x, (c + 1, m.1), a, s);
                            m.1 = c;
                            r
                        }
                        ("a", true) => {
                            let r = possibilities(&wfs, d, x, m, (a.0, c - 1), s);
                            a.0 = c;
                            r
                        }
                        ("a", false) => {
                            let r = possibilities(&wfs, d, x, m, (c + 1, a.1), s);
                            a.1 = c;
                            r
                        }
                        ("s", true) => {
                            let r = possibilities(&wfs, d, x, m, a, (s.0, c - 1));
                            s.0 = c;
                            r
                        }
                        ("s", false) => {
                            let r = possibilities(&wfs, d, x, m, a, (c + 1, s.1));
                            s.1 = c;
                            r
                        }
                        _ => unreachable!(),
                    }
                }
            }
        })
        .sum()
}
