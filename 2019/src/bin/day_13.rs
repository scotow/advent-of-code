advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> usize {
    let mut blocks = 0;
    prog.run();
    while prog.pull().is_some() {
        prog.pull();
        if prog.pull().unwrap() == 2 {
            blocks += 1;
        }
    }
    assert_eq!(blocks, 286);
    blocks
}

fn part_2(mut prog: Program) -> i64 {
    *prog.byte_mut(0) = 2;
    let mut pad = 0;
    let mut ball = 0;
    let mut score = 0;
    loop {
        match prog.run() {
            Status::Halted => break,
            Status::Paused => prog.push(match pad.cmp(&ball) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            }),
        }
        prog.run();
        while let Some(x) = prog.pull() {
            match (x, prog.pull().unwrap(), prog.pull().unwrap()) {
                (-1, 0, s) => score = s,
                (x, _, 3) => pad = x,
                (x, _, 4) => ball = x,
                _ => (),
            }
        }
    }
    assert_eq!(score, 14538);
    score
}
