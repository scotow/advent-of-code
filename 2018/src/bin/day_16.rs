advent_of_code_2018::main!();

fn generator(input: &str) -> (Vec<([u16; 4], [u16; 4], [u16; 4])>, Vec<[u16; 4]>) {
    let (samples, program) = input.split_once("\n\n\n\n").unwrap();
    (
        samples
            .split("\n\n")
            .map(|ex| {
                ex.lines()
                    .map(|l| {
                        l.split(['[', ',', ' ', ']'])
                            .filter_map(|n| n.parse().ok())
                            .collect_vec()
                            .try_into()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        program
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect(),
    )
}

fn part_1((samples, _): (Vec<([u16; 4], [u16; 4], [u16; 4])>, Vec<[u16; 4]>)) -> usize {
    samples
        .into_iter()
        .filter(|&(before, cmd, after)| {
            (0..=15)
                .filter(|&code| {
                    let mut regs = before;
                    exec(&mut regs, [code, cmd[1], cmd[2], cmd[3]]);
                    regs == after
                })
                .count()
                >= 3
        })
        .count()
}

fn part_2((samples, program): (Vec<([u16; 4], [u16; 4], [u16; 4])>, Vec<[u16; 4]>)) -> u16 {
    let mut mapping = HashMap::with_capacity(16);
    while mapping.len() < 16 {
        for &(before, cmd, after) in &samples {
            if let Ok((hidden, code)) = (0..=15)
                .filter_map(|code| {
                    if mapping.values().contains(&code) {
                        return None;
                    }
                    let mut regs = before;
                    exec(&mut regs, [code, cmd[1], cmd[2], cmd[3]]);
                    (regs == after).then(|| (cmd[0], code))
                })
                .exactly_one()
            {
                mapping.insert(hidden, code);
                break;
            }
        }
    }
    program.into_iter().fold([0; 4], |mut regs, cmd| {
        exec(&mut regs, [mapping[&cmd[0]], cmd[1], cmd[2], cmd[3]]);
        regs
    })[0]
}

fn exec(regs: &mut [u16; 4], op: [u16; 4]) {
    let p1 = match op[0] {
        0..=8 | 11 | 12 | 14 | 15 => regs[op[1] as usize],
        _ => op[1],
    };
    let p2 = match op[0] {
        0 | 2 | 4 | 6 | 10 | 12 | 13 | 15 => regs[op[2] as usize],
        _ => op[2],
    };
    regs[op[3] as usize] = match op[0] {
        0 | 1 => p1 + p2,
        2 | 3 => p1 * p2,
        4 | 5 => p1 & p2,
        6 | 7 => p1 | p2,
        8 | 9 => p1,
        10 | 11 | 12 => (p1 > p2) as u16,
        13 | 14 | 15 => (p1 == p2) as u16,
        _ => unreachable!(),
    }
}
