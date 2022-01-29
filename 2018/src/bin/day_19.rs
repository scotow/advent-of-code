advent_of_code_2018::main!();

fn generator(input: &str) -> (usize, Vec<[u32; 4]>) {
    let ops = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];
    let mut lines = input.lines();
    (
        lines
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap(),
        lines
            .map(|l| {
                let mut parts = l.split_whitespace();
                let op = parts.next().unwrap();
                let mut cmd = vec![ops.into_iter().position(|s| s == op).unwrap() as u32];
                cmd.extend(parts.map(|p| p.parse::<u32>().unwrap()));
                cmd.try_into().unwrap()
            })
            .collect(),
    )
}

fn part_1((ptr_idx, program): (usize, Vec<[u32; 4]>)) -> u32 {
    solve(ptr_idx, program, [0; 6])
}

fn part_2((ptr_idx, program): (usize, Vec<[u32; 4]>)) -> u32 {
    solve(ptr_idx, program, [1, 0, 0, 0, 0, 0])
}

fn solve(ptr_idx: usize, program: Vec<[u32; 4]>, mut vars: [u32; 6]) -> u32 {
    let mut previous = 0;
    let mut visited = 0;
    loop {
        let op = match program.get(vars[ptr_idx] as usize) {
            Some(&op) => op,
            None => {
                return vars[0];
            }
        };
        exec(&mut vars, op);
        vars[ptr_idx] += 1;
        if vars[4] == previous {
            visited += 1;
        }
        if visited == 16 {
            break;
        }
        previous = vars[4]
    }
    let mut factor = 0;
    for n in 1..=previous {
        if previous % n == 0 {
            factor += n;
        }
    }
    factor
}

fn exec(regs: &mut [u32; 6], op: [u32; 4]) {
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
        10 | 11 | 12 => (p1 > p2) as u32,
        13 | 14 | 15 => (p1 == p2) as u32,
        _ => unreachable!(),
    }
}
