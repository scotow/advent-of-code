advent_of_code_2019::main!();

fn generator(input: &str) -> HashMap<&str, (u32, Vec<(u32, &str)>)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l
                .split([',', '=', '>'])
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|p| {
                    let (n, t) = p.split_once(' ').unwrap();
                    (n.parse::<u32>().unwrap(), t)
                })
                .collect_vec();
            let (n, t) = parts.pop().unwrap();
            (t, (n, parts))
        })
        .collect()
}

fn part_1(receipts: HashMap<&'static str, (u32, Vec<(u32, &'static str)>)>) -> u32 {
    // dbg!(&input);
    let mut inventory = HashMap::new();
    let mut requirements = HashMap::new();
    requirements.insert("FUEL", 1);

    for _ in 0.. {
        // dbg!(&requirements);
        let (&next, &next_n) = requirements
            .iter()
            .filter(|&(&t, n)| t != "ORE")
            .next()
            .unwrap();
        // dbg!(next);

        let (produce, need) = &receipts[&next];
        for &(n, t) in need {
            *requirements.entry(t).or_default() += n;
        }
        *inventory.entry(next).or_default() += *produce;

        for (r, req_n) in requirements.iter_mut() {
            if let Some(inv_n) = inventory.get_mut(r) {
                let consume = (*req_n).min(*inv_n);
                *inv_n -= consume;
                *req_n -= consume;
            }
        }
        requirements.retain(|_, n| *n > 0);

        // dbg!(&inventory, &requirements);

        if requirements.len() == 1 && requirements.contains_key("ORE") {
            return requirements["ORE"];
        }
    }
    0
}

fn part_2(input: HashMap<&str, (u32, Vec<(u32, &str)>)>) -> u8 {
    0
}
