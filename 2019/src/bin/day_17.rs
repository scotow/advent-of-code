advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i16 {
    prog.run();
    let (mut x, mut y) = (0i16, 0);
    let mut cell = HashSet::new();
    while let Some(c) = prog.pull() {
        match c {
            35 => {
                cell.insert((x, y));
            }
            10 => {
                y += 1;
                x = -1;
            }
            _ => (),
        }
        x += 1;
    }
    cell.iter()
        .filter(|&&(x, y)| {
            [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                .into_iter()
                .all(|xy| cell.contains(&xy))
        })
        .map(|&(x, y)| (x * y).abs())
        .sum()
}

fn part_2(prog: Program) -> u8 {
    0
}
