advent_of_code_2019::main!();

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i16 {
    let main = [0, 0, 1, 1, 2, 1, 2, 1, 2, 0];
    let fs = [
        vec!["L10", "L10", "R6"],
        vec!["R12", "L12", "L12"],
        vec!["L6", "L10", "R12", "R12"],
    ];
    let path = main.into_iter().flat_map(|i| &fs[i]).copied().collect_vec();
    dbg!(&path);

    fn find_sub<'a>(path: &'a [&'a str]) -> &'a [&'a str] {
        (1..path.len())
            .rev()
            .map(|s| {
                (
                    &path[..s],
                    path[s..].windows(s).filter(|&w| w == &path[..s]).count(),
                )
            })
            .filter(|(sp, n)| (2..=4).contains(&sp.len()) && *n >= 2)
            .next()
            .unwrap()
            .0
    }

    fn skip_known<'a, 'b>(mut path: &'a [&'a str], fs: &'b [&'b [&'a str]]) -> &'a [&'a str] {
        loop {
            if let Some(f) = fs.into_iter().find(|f| path.starts_with(f)) {
                path = &path[f.len()..];
            } else {
                return path;
            }
        }
        // path
    }

    let mut remaining = path.as_slice();
    let f1 = find_sub(remaining);
    dbg!(f1);
    remaining = skip_known(remaining, &[f1]);

    let f2 = find_sub(remaining);
    dbg!(f2);
    remaining = skip_known(remaining, &[f1, f2]);

    let f3 = find_sub(remaining);
    dbg!(f3);
    0
}

// fn part_1(mut prog: Program) -> i16 {
//     // vec!["L", "10", "L", "10", "R", "6"],
//     // vec!["R", "12", "L", "12", "L", "12"],
//     // vec!["L", "6", "L", "10", "R", "12", "R", "12"],
//
//     let moves = vec!["L10", "R6", "R12", "L12", "L6"];
//     let f_generator = (3..=4).flat_map(|s| (0..s).map(|_| &moves).multi_cartesian_product());
//     // let it = f_generator.clone();
//     dbg!(iproduct!(
//         f_generator.clone(),
//         f_generator.clone(),
//         f_generator.clone()
//     )
//     .count());
//     // .collect_vec());
//     0
// }

// fn part_1(mut prog: Program) -> i16 {
//     prog.run();
//     let (mut x, mut y) = (0i16, 0);
//     let mut cell = HashSet::new();
//     while let Some(c) = prog.pull() {
//         match c {
//             35 => {
//                 cell.insert((x, y));
//             }
//             10 => {
//                 y += 1;
//                 x = -1;
//             }
//             _ => (),
//         }
//         x += 1;
//     }
//     cell.iter()
//         .filter(|&&(x, y)| {
//             [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
//                 .into_iter()
//                 .all(|xy| cell.contains(&xy))
//         })
//         .map(|&(x, y)| (x * y).abs())
//         .sum()
// }

#[allow(unstable_name_collisions)]
fn part_2(mut prog: Program) -> i64 {
    // *prog.byte_mut(0) = 2;
    // let main = [0, 0, 1, 1, 2, 1, 2, 1, 2, 0];
    // let functions = [
    //     vec!["L", "10", "L", "10", "R", "6"],
    //     vec!["R", "12", "L", "12", "L", "12"],
    //     vec!["L", "6", "L", "10", "R", "12", "R", "12"],
    // ];
    // for b in chain!(
    //     main.into_iter().map(|n| b'A' + n).intersperse(b','),
    //     once(b'\n'),
    //     functions
    //         .into_iter()
    //         .map(|f| f
    //             .into_iter()
    //             .intersperse(",")
    //             .flat_map(|i| i.bytes())
    //             .collect_vec())
    //         .intersperse(vec![b'\n'])
    //         .flatten(),
    //     "\nn\n".bytes(),
    // ) {
    //     prog.push(b as i64);
    // }
    // prog.run();
    // prog.pull_all().pop_back().unwrap()
    0
}
