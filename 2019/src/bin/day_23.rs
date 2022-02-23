advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> i64 {
    let mut computers = build_network(prog);
    loop {
        for n in 0..50 {
            let computer = computers.get_mut(&n).unwrap();
            computer.push(-1);
            computer.run();
            for (t, x, y) in computer.pull_all().into_iter().tuples() {
                if t == 255 {
                    return y;
                }
                computers.get_mut(&t).unwrap().push_multiple([x, y]);
            }
        }
    }
}

fn part_2(prog: Program) -> i64 {
    let mut computers = build_network(prog);
    let mut nat = (0, 0);
    let mut last_y = 0;
    loop {
        let mut delivered = 0;
        for n in 0..50 {
            let computer = computers.get_mut(&n).unwrap();
            computer.push(-1);
            computer.run();
            for (t, x, y) in computer.pull_all().into_iter().tuples() {
                delivered += 1;
                if t == 255 {
                    nat = (x, y);
                } else {
                    computers.get_mut(&t).unwrap().push_multiple([x, y]);
                }
            }
        }
        if delivered == 0 {
            if last_y == nat.1 {
                return last_y;
            }
            last_y = nat.1;
            computers.get_mut(&0).unwrap().push_multiple([nat.0, nat.1]);
        }
    }
}

fn build_network(prog: Program) -> HashMap<i64, Program> {
    (0..50)
        .map(|n| {
            let mut prog = prog.clone();
            prog.push(n);
            prog.run();
            (n, prog)
        })
        .collect()
}
