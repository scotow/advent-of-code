use num_integer::Integer;
advent_of_code_2019::main!();

fn generator(input: &str) -> HashMap<&str, (u64, Vec<(u64, &str)>)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l
                .split([',', '=', '>'])
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|p| {
                    let (n, t) = p.split_once(' ').unwrap();
                    (n.parse::<u64>().unwrap(), t)
                })
                .collect_vec();
            let (n, t) = parts.pop().unwrap();
            (t, (n, parts))
        })
        .collect()
}

fn part_1(receipts: HashMap<&str, (u64, Vec<(u64, &str)>)>) -> u64 {
    solve(&receipts, 1)
}

fn part_2(receipts: HashMap<&str, (u64, Vec<(u64, &str)>)>) -> u64 {
    let (mut min, mut max) = (1, 1_000_000_000_000);
    loop {
        let middle = (min + max).div_ceil(&2);
        if solve(&receipts, middle) > 1_000_000_000_000 {
            max = middle - 1;
        } else {
            min = middle;
        }
        if min == max {
            return max;
        }
    }
}

fn solve<'a>(receipts: &HashMap<&'a str, (u64, Vec<(u64, &'a str)>)>, fuels: u64) -> u64 {
    let mut inventory = HashMap::new();
    let mut requirements = HashMap::from([("FUEL", fuels)]);
    while requirements.len() > 1 || !requirements.contains_key("ORE") {
        let (&next, &next_n) = requirements
            .iter()
            .filter(|&(&t, _)| t != "ORE")
            .next()
            .unwrap();
        let (produce, need) = &receipts[&next];
        let times = next_n.div_ceil(produce);
        for &(n, t) in need {
            *requirements.entry(t).or_default() += n * times;
        }
        *inventory.entry(next).or_default() += *produce * times;
        for (r, req_n) in requirements.iter_mut() {
            if let Some(inv_n) = inventory.get_mut(r) {
                let consume = (*req_n).min(*inv_n);
                *inv_n -= consume;
                *req_n -= consume;
            }
        }
        requirements.retain(|_, n| *n > 0);
    }
    requirements["ORE"]
}
