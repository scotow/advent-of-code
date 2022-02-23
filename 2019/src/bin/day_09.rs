advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> i64 {
    solve(prog, 1)
}

fn part_2(prog: Program) -> i64 {
    solve(prog, 2)
}

fn solve(mut prog: Program, input: i64) -> i64 {
    prog.push(input);
    prog.run();
    prog.pull().unwrap()
}
