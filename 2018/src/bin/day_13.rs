advent_of_code_2018::main!();

#[derive(Copy, Clone, Debug)]
enum Dir {
    Horizontal,
    Vertical,
    Turn1,
    Turn2,
    Intersection,
}

fn generator(input: &str) -> HashMap<(isize, isize), Dir> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().filter_map(move |(x, c)| {
                Some((
                    (x as isize, y as isize),
                    match c {
                        b'-' => Dir::Horizontal,
                        b'|' => Dir::Vertical,
                        b'/' => Dir::Turn1,
                        b'\\' => Dir::Turn2,
                        b'+' => Dir::Intersection,
                        _ => return None,
                    },
                ))
            })
        })
        .collect()
}

fn part_1(input: HashMap<(isize, isize), Dir>) -> u8 {
    let mut cart = (2, 0);
    let mut dir = (1, 0);

    dbg!(&input);
    0
}

fn part_2(input: HashMap<(isize, isize), Dir>) -> u8 {
    0
}
