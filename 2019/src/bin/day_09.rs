advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> i64 {
    let out = solve(prog, 1);
    assert_eq!(out, 3241900951);
    out
}

fn part_2(prog: Program) -> i64 {
    let out = solve(prog, 2);
    assert_eq!(out, 83089);
    out
}

fn solve(mut prog: Program, input: i64) -> i64 {
    prog.push(input);
    prog.run();
    prog.pull().unwrap()
}
