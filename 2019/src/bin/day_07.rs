advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|phases| {
            phases.into_iter().fold(0, |signal, phase| {
                let mut prog = prog.clone();
                prog.push(phase);
                prog.push(signal);
                prog.run();
                prog.pull().unwrap()
            })
        })
        .max()
        .unwrap()
}

fn part_2(prog: Program) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut progs = phases
                .into_iter()
                .map(|phase| {
                    let mut prog = prog.clone();
                    prog.push(phase);
                    prog
                })
                .collect_vec();
            (0..5)
                .cycle()
                .fold_while(0, |signal, pi| {
                    progs[pi].push(signal);
                    let status = progs[pi].run();
                    let next = progs[pi].pull().unwrap();
                    if matches!(status, Status::Halted) && pi == 4 {
                        FoldWhile::Done(next)
                    } else {
                        FoldWhile::Continue(next)
                    }
                })
                .into_inner()
        })
        .max()
        .unwrap()
}
