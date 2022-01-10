advent_of_code_2017::main!();

#[derive(Copy, Clone, Debug)]
enum Op {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Op {
    fn exec(self, programs: &mut [u8; 16]) {
        match self {
            Op::Spin(n) => programs.rotate_right(n),
            Op::Exchange(i, j) => programs.swap(i, j),
            Op::Partner(c1, c2) => Op::Exchange(
                programs.iter().position(|&c| c == c1).unwrap(),
                programs.iter().position(|&c| c == c2).unwrap(),
            )
            .exec(programs),
        }
    }
}

fn generator(input: &str) -> Vec<Op> {
    input
        .split(',')
        .map(|op| {
            let args = op[1..].split('/').collect_vec();
            match op.as_bytes()[0] {
                b's' => Op::Spin(args[0].parse().unwrap()),
                b'x' => Op::Exchange(args[0].parse().unwrap(), args[1].parse().unwrap()),
                b'p' => Op::Partner(args[0].as_bytes()[0] - b'a', args[1].as_bytes()[0] - b'a'),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(input: Vec<Op>) -> String {
    let mut programs: [u8; 16] = (0..16).collect_vec().try_into().unwrap();
    dance(&mut programs, &input);
    programs.into_iter().map(|c| (b'a' + c) as char).collect()
}

fn part_2(input: Vec<Op>) -> String {
    let mut programs: [u8; 16] = (0..16).collect_vec().try_into().unwrap();
    let mut cache = HashMap::new();
    let mut i = 0usize;
    loop {
        if i == 1_000_000_000 {
            return programs.into_iter().map(|c| (b'a' + c) as char).collect();
        }
        if let Some(&prev) = cache.get(&programs) {
            let jump = i - prev;
            let n_jump = (1_000_000_000 - i) / jump;
            if n_jump >= 1 {
                i += jump * n_jump;
                continue;
            }
        }
        cache.insert(programs, i);
        dance(&mut programs, &input);
        i += 1;
    }
}

fn dance(programs: &mut [u8; 16], ops: &[Op]) {
    for &op in ops {
        op.exec(programs);
    }
}
