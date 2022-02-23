advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> usize {
    iproduct!(0..50, 0..50)
        .filter(|&(x, y)| affected(&prog, x, y))
        .count()
}

fn part_2(prog: Program) -> i64 {
    let (mut x, mut y) = (0, 50);
    loop {
        while !affected(&prog, x, y) {
            x += 1;
        }
        if let Some(x) = (x..)
            .take_while(|&x| affected(&prog, x + 99, y))
            .find(|&x| affected(&prog, x, y + 99))
        {
            return x * 10_000 + y;
        }
        y += 1;
    }
}

fn affected(prog: &Program, x: i64, y: i64) -> bool {
    let mut prog = prog.clone();
    prog.push_multiple([x, y]);
    prog.run();
    prog.pull().unwrap() == 1
}
