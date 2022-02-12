advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i64 {
    *prog.byte_mut(1) = 12;
    *prog.byte_mut(2) = 2;
    prog.run();
    assert_eq!(prog.byte(0), 5098658);
    prog.byte(0)
}

fn part_2(prog: Program) -> i64 {
    let (n, v) = iproduct!(0..=99, 0..=99)
        .find(|&(n, v)| {
            let mut prog = prog.clone();
            *prog.byte_mut(1) = n;
            *prog.byte_mut(2) = v;
            prog.run();
            prog.byte(0) == 19690720
        })
        .unwrap();
    assert_eq!(n * 100 + v, 5064);
    n * 100 + v
}
