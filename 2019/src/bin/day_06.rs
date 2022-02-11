advent_of_code_2019::main!();

fn generator(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .map(|l| {
            let (o, p) = l.split_once(')').unwrap();
            (p, o)
        })
        .collect()
}

fn part_1(input: HashMap<&str, &str>) -> usize {
    input.keys().map(|p| distance(&input, p, "COM")).sum()
}

fn part_2(input: HashMap<&str, &str>) -> usize {
    let p2 = path(&input, "SAN");
    for p in path(&input, "YOU") {
        if p2.contains(&p) {
            return distance(&input, "YOU", p) + distance(&input, "SAN", p) - 2;
        }
    }
    unreachable!()
}

fn distance(map: &HashMap<&str, &str>, planet: &str, to: &str) -> usize {
    if planet == to {
        0
    } else {
        1 + distance(map, map[planet], to)
    }
}

fn path<'a>(map: &HashMap<&'a str, &'a str>, mut planet: &'a str) -> Vec<&'a str> {
    let mut p = Vec::new();
    while planet != "COM" {
        planet = map[planet];
        p.push(planet);
    }
    p
}
