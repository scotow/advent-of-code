advent_of_code_2018::main!();

#[derive(Clone, Debug)]
struct Group {
    team: u8,
    units: u32,
    hp: u32,
    ap: u32,
    attack: &'static str,
    init: u32,
    weak: Vec<&'static str>,
    immune: Vec<&'static str>,
}

impl Group {
    fn parse(s: &'static str, team: u8) -> Self {
        let parts = s.split_whitespace().collect_vec();
        let mut weak = Vec::new();
        let mut immune = Vec::new();
        let mut shift = 0;
        if s.contains('(') {
            let specs = s.trim_matches(|c| !['(', ')'].contains(&c));
            let specs = &specs[1..specs.len() - 1];
            for s in specs.split("; ") {
                let (to, _, kinds) = s.splitn(3, ' ').collect_tuple().unwrap();
                *match to {
                    "weak" => &mut weak,
                    "immune" => &mut immune,
                    _ => unreachable!(),
                } = kinds.split(", ").collect_vec();
            }
            shift = specs.split_whitespace().count();
        }
        Group {
            team,
            units: parts[0].parse().unwrap(),
            hp: parts[4].parse().unwrap(),
            ap: parts[12 + shift].parse().unwrap(),
            attack: parts[13 + shift],
            init: parts[17 + shift].parse().unwrap(),
            weak,
            immune,
        }
    }

    fn ep(&self) -> u32 {
        self.units * self.ap
    }

    fn dmg(&self, to: &Group) -> u32 {
        let mut dmg = self.units * self.ap;
        if to.weak.contains(&self.attack) {
            dmg *= 2;
        } else if to.immune.contains(&self.attack) {
            dmg = 0;
        }
        dmg
    }
}

fn generator(input: &'static str) -> HashMap<usize, Group> {
    input
        .split("\n\n")
        .enumerate()
        .flat_map(|(n, team)| team.lines().skip(1).map(move |l| Group::parse(l, n as u8)))
        .enumerate()
        .collect()
}

fn part_1(groups: HashMap<usize, Group>) -> u32 {
    solve(groups).unwrap().1
}

fn part_2(groups: HashMap<usize, Group>) -> u32 {
    (1..)
        .find_map(|boost| {
            let mut groups = groups.clone();
            groups
                .values_mut()
                .filter(|g| g.team == 0)
                .for_each(|g| g.ap += boost);
            solve(groups).and_then(|(w, u)| (w == 0).then(|| u))
        })
        .unwrap()
}

fn solve(mut groups: HashMap<usize, Group>) -> Option<(u8, u32)> {
    let mut units = groups.values().map(|g| g.units).sum();
    while !groups.values().map(|g| g.team).all_equal() {
        let mut targeting = HashMap::new();
        for (&an, ag) in groups.iter().sorted_by(|(_, g1), (_, g2)| {
            g1.ep().cmp(&g2.ep()).then(g1.init.cmp(&g2.init)).reverse()
        }) {
            groups
                .iter()
                .filter(|(dn, dg)| {
                    ag.team != dg.team && !targeting.values().contains(dn) && ag.dmg(dg) > 0
                })
                .sorted_by(|(_, g1), (_, g2)| {
                    ag.dmg(g1)
                        .cmp(&ag.dmg(g2))
                        .then(g1.ep().cmp(&g2.ep()))
                        .then(g1.init.cmp(&g2.init))
                        .reverse()
                })
                .next()
                .and_then(|(&dn, _)| targeting.insert(an, dn));
        }
        for an in groups
            .iter()
            .sorted_by_key(|(_, g)| g.init)
            .map(|(&an, _)| an)
            .rev()
            .collect_vec()
        {
            if !groups.contains_key(&an) || !targeting.contains_key(&an) {
                continue;
            }
            let dg = &groups[&targeting[&an]];
            let deaths = groups[&an].dmg(dg) / dg.hp;
            if deaths >= dg.units {
                groups.remove(&targeting[&an]);
            } else {
                groups.get_mut(&targeting[&an]).unwrap().units -= deaths;
            }
        }
        let new_units = groups.values().map(|g| g.units).sum();
        if new_units == units {
            return None;
        }
        units = new_units;
    }
    Some((groups.values().next().unwrap().team, units))
}
