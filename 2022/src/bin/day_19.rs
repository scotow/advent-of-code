advent_of_code_2022::main!();

#[derive(Clone, Debug)]
struct Blueprint {
    ore: u16,
    clay: u16,
    obsidian: (u16, u16),
    geode: (u16, u16),
}

#[derive(Default, Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Collection {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl Add for Collection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Collection {
    fn craft(&self, robots: &Collection, bp: &Blueprint) -> impl Iterator<Item = (Self, Self)> {
        if self.ore >= bp.geode.0 && self.obsidian >= bp.geode.1 {
            let (mut new_inv, mut built) = (*self, Collection::default());
            new_inv.ore -= bp.geode.0;
            new_inv.obsidian -= bp.geode.1;
            built.geode = 1;
            vec![(new_inv, built)].into_iter()
        } else {
            let mut outs = Vec::with_capacity(3);
            if self.ore >= bp.obsidian.0
                && self.clay >= bp.obsidian.1 && robots.obsidian < bp.geode.1
            {
                let (mut new_inv, mut built) = (*self, Collection::default());
                new_inv.ore -= bp.obsidian.0;
                new_inv.clay -= bp.obsidian.1;
                built.obsidian = 1;
                outs.push((new_inv, built))
            }
            if self.ore >= bp.clay && robots.clay < bp.obsidian.1 {
                let (mut new_inv, mut built) = (*self, Collection::default());
                new_inv.ore -= bp.clay;
                built.clay = 1;
                outs.push((new_inv, built))
            }
            if self.ore >= bp.ore && robots.ore
                < [bp.ore, bp.clay, bp.obsidian.0, bp.geode.0]
                .into_iter()
                .max()
                .unwrap() {
                let (mut new_inv, mut built) = (*self, Collection::default());
                new_inv.ore -= bp.ore;
                built.ore = 1;
                outs.push((new_inv, built))
            }
            if outs.len() != 3 {
                outs.push((*self, Collection::default()));
            }
            outs.into_iter()
        }
    }
}

fn generator(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let mut ws = l.split_whitespace().filter_map(|w| w.parse().ok());
            Blueprint {
                ore: ws.next().unwrap(),
                clay: ws.next().unwrap(),
                obsidian: ws.next_tuple().unwrap(),
                geode: ws.next_tuple().unwrap(),
            }
        })
        .collect()
}

fn part_1(blueprints: Vec<Blueprint>) -> u16 {
    blueprints
        .into_iter()
        .map(solve::<24>)
        .enumerate()
        .map(|(i, r)| (i + 1) as u16 * r)
        .sum()
}

fn part_2(blueprints: Vec<Blueprint>) -> u16 {
    blueprints
        .into_iter()
        .take(3)
        .map(solve::<32>)
        .product()
}

fn solve<const M: u8>(blueprint: Blueprint) -> u16 {
    let mut seen = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert((Collection::default(), Collection {
        ore: 1,
        ..Default::default()
    }));

    let mut min = 0;
    loop {
        let mut next_queue = HashSet::with_capacity(queue.len() * 2);
        for (inv, robots) in queue {
            for (mut new_inv, built) in inv.craft(&robots, &blueprint) {
                new_inv = new_inv + robots;
                let new_robots = robots + built;
                if seen.insert((new_inv, new_robots)) {
                    next_queue.insert((new_inv, new_robots));
                }
            }
        }
        min += 1;
        if min == M {
            return next_queue.into_iter().map(|(inv, _)| inv.geode).max().unwrap();
        }
        let max_geode = next_queue.iter().map(|(inv, _)| inv.geode).max().unwrap();
        next_queue.retain(|(inv, _)| inv.geode == max_geode);
        queue = next_queue;
    }
}
