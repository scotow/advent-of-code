advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i64 {
    prog.push(1);
    prog.run();
    loop {
        let code = prog.pull().unwrap();
        if code != 0 {
            return code;
        }
    }
}

fn part_2(mut prog: Program) -> i64 {
    prog.push(5);
    prog.run();
    prog.pull().unwrap()
}
