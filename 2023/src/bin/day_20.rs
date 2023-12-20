advent_of_code_2023::main!();

#[derive(Clone, Debug)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, Pulse>),
}

#[derive(Copy, Clone, Debug)]
enum Pulse {
    Low,
    High,
}

fn generator(input: &str) -> HashMap<&str, (Module, Vec<&str>)> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|m| {
            let (l, r) = m.split_once(" -> ").unwrap();
            let out = r.split(", ").collect();
            match l.as_bytes()[0] {
                b'%' => (&l[1..], (Module::FlipFlop(false), out)),
                b'&' => (&l[1..], (Module::Conjunction(HashMap::new()), out)),
                _ => (l, (Module::Broadcaster, out)),
            }
        })
        .collect();
    for (k, (_, out)) in modules.clone() {
        for o in out {
            if let Some((Module::Conjunction(ins), _)) = modules.get_mut(o) {
                ins.insert(k, Pulse::Low);
            }
        }
    }
    modules
}

fn part_1(modules: HashMap<&str, (Module, Vec<&str>)>) -> u64 {
    solve(modules, Some(1_000)).0
}

fn part_2(modules: HashMap<&str, (Module, Vec<&str>)>) -> u64 {
    solve(modules, None).1.unwrap()
}

fn solve(mut modules: HashMap<&str, (Module, Vec<&str>)>, max: Option<u64>) -> (u64, Option<u64>) {
    let (mut low, mut high) = (0, 0);
    let rx_prev = modules
        .iter()
        .filter_map(|(&k, (_, outs))| outs.contains(&"rx").then_some(k))
        .exactly_one()
        .unwrap();
    let mut meta_modules = modules
        .iter()
        .filter_map(|(&k, (_, outs))| outs.contains(&rx_prev).then_some((k, None)))
        .collect::<HashMap<_, _>>();
    for i in 1..=max.unwrap_or(u64::MAX) {
        let mut queue = vec![("broadcaster", None, Pulse::Low)];
        while !queue.is_empty() {
            for (name, origin, pulse) in take(&mut queue) {
                *match pulse {
                    Pulse::Low => &mut low,
                    Pulse::High => &mut high,
                } += 1;
                if matches!(pulse, Pulse::Low) && meta_modules.get(&name) == Some(&None) {
                    meta_modules.insert(name, Some(i));
                    if meta_modules.values().all(|m| matches!(m, Some(_))) {
                        return (
                            low * high,
                            meta_modules.into_values().map(|m| m.unwrap()).reduce(lcm),
                        );
                    }
                }
                let Some((module, outs)) = modules.get_mut(name) else {
                    continue;
                };
                match module {
                    Module::Broadcaster => {
                        queue.extend(outs.iter().map(|&o| (o, Some(name), pulse)))
                    }
                    Module::FlipFlop(state) if matches!(pulse, Pulse::Low) => {
                        *state = !*state;
                        queue.extend(
                            outs.iter().map(|&o| {
                                (o, Some(name), [Pulse::Low, Pulse::High][*state as usize])
                            }),
                        );
                    }
                    Module::Conjunction(ins) => {
                        *ins.get_mut(origin.unwrap()).unwrap() = pulse;
                        queue.extend(outs.iter().map(|&o| {
                            (
                                o,
                                Some(name),
                                [Pulse::High, Pulse::Low]
                                    [ins.values().all(|p| matches!(p, Pulse::High)) as usize],
                            )
                        }));
                    }
                    _ => (),
                }
            }
        }
    }
    (low * high, None)
}
