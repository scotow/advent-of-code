advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<(i32, i32, i32, i32)> {
    input
        .lines()
        .map(|l| {
            l.split(['<', ',', '>', '='])
                .filter_map(|p| p.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(i32, i32, i32, i32)>) -> usize {
    let (sx, sy, sz, r) = *input.iter().max_by_key(|(_, _, _, r)| r).unwrap();
    input
        .into_iter()
        .filter(|&(x, y, z, _)| abs_diff(sx, x) + abs_diff(sy, y) + abs_diff(sz, z) <= r)
        .count()
}

fn part_2(input: Vec<(i32, i32, i32, i32)>) -> u8 {
    let groups = input
        .into_iter()
        .powerset()
        .filter(|bots| bots.len() >= 2)
        .filter(|bots| {
            let (x1, y1, z1, r1) = bots[0];
            bots.into_iter().skip(1).all(|&(x2, y2, z2, r2)| {
                abs_diff(x1, x2) + abs_diff(y1, y2) + abs_diff(z1, z2) <= r1 + r2
            })
        })
        .collect_vec();
    dbg!(&groups);
    0
}

// fn part_2(input: Vec<(i32, i32, i32, i32)>) -> u8 {
//     let groups = input
//         .into_iter()
//         .powerset()
//         .filter(|bots| bots.len() >= 2)
//         .filter(|bots| {
//             let (x, y, z, r) = bots[0];
//             let (ax1, ax2, ay1, ay2, az1, az2) = (x - r, x + r, y - r, y + r, z - r, z + r);
//             bots.into_iter().skip(1).all(|&(x, y, z, r)| {
//                 let (bx1, bx2, by1, by2, bz1, bz2) = (x - r, x + r, y - r, y + r, z - r, z + r);
//                 ax1 < bx2 && ax2 > bx1 && ay1 < by2 && ay2 > by1 && az1 < bz2 && az2 > bz1
//             })
//         })
//         .collect_vec();
//     dbg!(&groups);
//     0
// }
