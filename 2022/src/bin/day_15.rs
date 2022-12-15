advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<(Pos<i64>, Pos<i64>)> {
    input
        .lines()
        .map(|l| {
            let mut p = l.split(['=', ':', ',']).filter_map(|n| n.parse().ok());
            (p.next_tuple().unwrap(), p.next_tuple().unwrap())
        })
        .collect()
}

fn part_1(input: Vec<(Pos<i64>, Pos<i64>)>) -> usize {
    let mut points = HashSet::new();
    for ((sx, sy), (bx, by)) in input {
        let radius = sx.abs_diff(bx) + sy.abs_diff(by);
        if let Some(on_line) = radius.checked_sub(sy.abs_diff(2_000_000)) {
            points.extend(
                (sx - on_line as i64..=sx + on_line as i64)
                    .filter(|&nbx| !(by == 2_000_000 && nbx == bx)),
            );
        }
    }
    points.len()
}

fn part_2(input: Vec<(Pos<i64>, Pos<i64>)>) -> i64 {
    let limit = 0..=4_000_000;
    for &((sx, sy), (bx, by)) in &input {
        let radius = sx.abs_diff(bx) + sy.abs_diff(by);
        let (mut px, mut py) = (sx, sy - (radius + 1) as i64);
        for (dx, dy) in [(1, 1), (-1, 1), (-1, -1), (1, -1)] {
            for _ in 0..=radius {
                px += dx;
                py += dy;
                if !limit.contains(&px) || !limit.contains(&py) {
                    continue;
                }
                if input.iter().all(|&((sx2, sy2), (bx2, by2))| {
                    sx2.abs_diff(px) + sy2.abs_diff(py) > sx2.abs_diff(bx2) + sy2.abs_diff(by2)
                }) {
                    return px * 4_000_000 + py;
                }
            }
        }
    }
    unreachable!()
}
