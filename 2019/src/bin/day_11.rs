advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(prog: Program) -> usize {
    solve(prog, false).len()
}

fn part_2(prog: Program) -> String {
    let grid = solve(prog, true);
    let (xm, xn) = grid.keys().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (ym, yn) = grid.keys().map(|&(_, y)| y).minmax().into_option().unwrap();
    (ym..=yn)
        .map(|y| {
            (xm..=xn)
                .map(|x| {
                    (grid.get(&(x, y)).unwrap_or(&0) == &1)
                        .then(|| '#')
                        .unwrap_or(' ')
                })
                .collect::<String>()
        })
        .join("\n")
}

fn solve(mut prog: Program, white_start: bool) -> HashMap<(isize, isize), i64> {
    let mut grid = HashMap::new();
    if white_start {
        grid.insert((0, 0), 1);
    }
    let mut pos = (0, 0);
    let mut dir = (0, -1);
    loop {
        prog.push(*grid.get(&pos).unwrap_or(&0));
        let status = prog.run();
        let paint = prog.pull().unwrap();
        if *grid.get(&pos).unwrap_or(&0) != paint {
            grid.insert(pos, paint);
        }
        let turn = prog.pull().unwrap();
        if turn == 0 {
            dir = (dir.1, -dir.0);
        } else {
            dir = (-dir.1, dir.0);
        }
        pos.0 += dir.0;
        pos.1 += dir.1;
        if matches!(status, Status::Halted) {
            break;
        }
    }
    grid
}
