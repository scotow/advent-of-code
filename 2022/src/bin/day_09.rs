advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<((i16, i16), i16)> {
    input
        .lines()
        .map(|l| {
            let (d, n) = l.split_once(' ').unwrap();
            (
                match d {
                    "U" => (0, -1),
                    "R" => (1, 0),
                    "D" => (0, 1),
                    "L" => (-1, 0),
                    _ => unreachable!(),
                },
                n.parse().unwrap(),
            )
        })
        .collect()
}

fn part_1(ops: Vec<((i16, i16), i16)>) -> usize {
    solve::<2>(ops)
}

fn part_2(ops: Vec<((i16, i16), i16)>) -> usize {
    solve::<10>(ops)
}

fn solve<const N: usize>(ops: Vec<((i16, i16), i16)>) -> usize {
    let mut knots = [(0, 0); N];
    let mut visited = HashSet::from([(0, 0)]);
    for ((dx, dy), f) in ops {
        for _ in 0..f {
            knots[0] = (knots[0].0 + dx, knots[0].1 + dy);
            for i in 1..N {
                follow(knots[i - 1], &mut knots[i]);
            }
            visited.insert(knots[N - 1]);
        }
    }
    visited.len()
}

fn follow((tx, ty): (i16, i16), (kx, ky): &mut (i16, i16)) {
    if tx.abs_diff(*kx) >= 2 || ty.abs_diff(*ky) >= 2 {
        (*kx, *ky) = neighbors8(*kx, *ky)
            .min_by_key(|&(nx, ny)| tx.abs_diff(nx) + ty.abs_diff(ny))
            .unwrap();
    }
}
