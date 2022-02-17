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
    let res = cell
        .iter()
        .filter(|&&(x, y)| {
            [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                .into_iter()
                .all(|xy| cell.contains(&xy))
        })
        .map(|&(x, y)| (x * y).abs())
        .sum();
    assert_eq!(res, 11140);
    res
}

#[allow(unstable_name_collisions)]
fn part_2(mut prog: Program) -> i64 {
    *prog.byte_mut(0) = 2;
    let main = [0, 0, 1, 1, 2, 1, 2, 1, 2, 0];
    let functions = [
        vec!["L", "10", "L", "10", "R", "6"],
        vec!["R", "12", "L", "12", "L", "12"],
        vec!["L", "6", "L", "10", "R", "12", "R", "12"],
    ];
    for b in chain!(
        main.into_iter().map(|n| b'A' + n).intersperse(b','),
        once(b'\n'),
        functions
            .into_iter()
            .map(|f| f
                .into_iter()
                .intersperse(",")
                .flat_map(|i| i.bytes())
                .collect_vec())
            .intersperse(vec![b'\n'])
            .flatten(),
        "\nn\n".bytes(),
    ) {
        prog.push(b as i64);
    }
    prog.run();
    let res = prog.pull_all().pop_back().unwrap();
    assert_eq!(res, 1113108);
    res
}
