advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> i64 {
    solve(
        prog,
        &["OR A T", "AND B T", "AND C T", "NOT T J", "AND D J", "WALK"],
    )
}

fn part_2(prog: Program) -> i64 {
    solve(
        prog,
        &[
            "OR A T", "AND B T", "AND C T", "NOT T J", "AND D J", "OR H T", "OR E T", "AND T J",
            "RUN",
        ],
    )
}

fn solve(mut prog: Program, ops: &[&str]) -> i64 {
    prog.push_multiple(
        ops.into_iter()
            .join("\n")
            .bytes()
            .chain(once(b'\n'))
            .map_into(),
    );
    prog.run();
    prog.pull_all().pop_back().unwrap()
}
