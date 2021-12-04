advent_of_code_2021::main!();

fn generator(input: &str) -> (Vec<i32>, Vec<[[i32; 5]; 5]>) {
    let mut input = input.split("\n\n");
    (
        input
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect(),
        input
            .map(|g| {
                g.lines()
                    .map(|l| {
                        l.split_whitespace()
                            .map(|n| n.parse().unwrap())
                            .collect_vec()
                            .try_into()
                            .unwrap()
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect(),
    )
}

fn part_1((outs, mut grids): (Vec<i32>, Vec<[[i32; 5]; 5]>)) -> i32 {
    for o in outs {
        apply_out(o, &mut grids);
        if let Some(w) = grids
            .iter()
            .find(|&g| is_won(g)) {
            return w.iter().flatten().filter(|c| **c != -1).sum::<i32>() * o;
        }
    };
    unreachable!();
}

fn part_2((outs, mut grids): (Vec<i32>, Vec<[[i32; 5]; 5]>)) -> i32 {
    for o in outs {
        apply_out(o, &mut grids);
        if grids.len() == 1 {
            if is_won(&grids[0]) {
                return grids[0].iter().flatten().filter(|c| **c != -1).sum::<i32>() * o;
            }
        } else {
            grids.retain(|g| !is_won(g));
        }
    };
    unreachable!();
}

fn apply_out(o: i32, grids: &mut[[[i32; 5]; 5]]) {
    grids.iter_mut().flatten().flatten().filter(|c| **c == o).for_each(|g| *g = -1);
}

fn is_won(grid: &[[i32; 5]]) -> bool {
    grid.iter().any(|r| r.iter().sum::<i32>() == -5) ||
        (0..5).map(|x| (0..5).map(move |y| grid[y][x])).any(|r| r.sum::<i32>() == -5)
}
