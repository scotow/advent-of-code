advent_of_code_2019::main!();

type Pos = (i16, i16);

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn parse_program(mut prog: Program) -> (HashSet<Pos>, Pos, Pos) {
    prog.run();
    let (mut x, mut y) = (0i16, 0);
    let mut start = ((0, 0), (0, 0));
    let mut cells = HashSet::new();
    while let Some(c) = prog.pull() {
        match c as u8 {
            b'#' => {
                cells.insert((x, y));
            }
            b'^' => {
                start = ((x, y), (0, -1));
            }
            b'>' => {
                start = ((x, y), (1, 0));
            }
            b'v' => {
                start = ((x, y), (0, 1));
            }
            b'<' => {
                start = ((x, y), (-1, 0));
            }
            b'\n' => {
                y += 1;
                x = -1;
            }
            _ => (),
        }
        x += 1;
    }
    (cells, start.0, start.1)
}

fn part_1(prog: Program) -> i16 {
    let cells = parse_program(prog).0;
    cells
        .iter()
        .filter(|&&(x, y)| {
            [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                .into_iter()
                .all(|xy| cells.contains(&xy))
        })
        .map(|&(x, y)| (x * y).abs())
        .sum()
}

#[allow(unstable_name_collisions)]
fn part_2(mut prog: Program) -> i64 {
    let (cells, mut pos, mut dir) = parse_program(prog.clone());
    let mut path = Vec::new();
    let mut n = 0;
    loop {
        if !cells.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            if n > 0 {
                path.push(n.to_string());
                n = 0;
            }
            if let Some((n_dir, c)) = next_turn(&cells, pos, dir) {
                path.push(String::from_utf8(vec![c]).unwrap());
                dir = n_dir;
            } else {
                break;
            }
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        n += 1;
    }
    let path = path
        .chunks_exact(2)
        .map(|c| {
            c.into_iter()
                .map(|s| s.as_str())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec();
    let mut remaining = path.as_slice();
    let mut main = Vec::new();
    let mut fs = Vec::new();
    for _ in 0..3 {
        let f = find_sub(remaining);
        fs.push(f);
        let (r, idx) = skip_known(remaining, &fs);
        remaining = r;
        main.extend(idx);
    }
    *prog.byte_mut(0) = 2;
    prog.push_multiple(
        chain!(
            once(
                main.into_iter()
                    .map(|n| ((b'A' + n as u8) as char))
                    .join(",")
            ),
            fs.into_iter().map(|f| f.into_iter().flatten().join(",")),
            once("n\n".to_owned()),
        )
        .intersperse("\n".to_owned())
        .flat_map(|s| s.into_bytes().into_iter().map(|b| b as i64)),
    );
    prog.run();
    prog.pull_all().pop_back().unwrap()
}

fn next_turn(cells: &HashSet<Pos>, pos: Pos, dir: Pos) -> Option<(Pos, u8)> {
    if cells.contains(&(pos.0 + dir.1, pos.1 - dir.0)) {
        Some(((dir.1, -dir.0), b'L'))
    } else if cells.contains(&(pos.0 - dir.1, pos.1 + dir.0)) {
        Some(((-dir.1, dir.0), b'R'))
    } else {
        None
    }
}

fn find_sub<'a>(path: &'a [[&'a str; 2]]) -> &'a [[&'a str; 2]] {
    (1..path.len())
        .rev()
        .map(|s| {
            (
                &path[..s],
                path[s..].windows(s).filter(|&w| w == &path[..s]).count(),
            )
        })
        .filter(|(sp, n)| (2..=5).contains(&sp.len()) && *n >= 2)
        .next()
        .unwrap()
        .0
}

fn skip_known<'a, 'b>(
    mut path: &'a [[&'a str; 2]],
    fs: &'b [&'b [[&'a str; 2]]],
) -> (&'a [[&'a str; 2]], Vec<usize>) {
    let mut indexes = Vec::new();
    loop {
        if let Some((i, f)) = fs
            .into_iter()
            .enumerate()
            .find(|(_, f)| path.starts_with(f))
        {
            path = &path[f.len()..];
            indexes.push(i);
        } else {
            return (path, indexes);
        }
    }
}
