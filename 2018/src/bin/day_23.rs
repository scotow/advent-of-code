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
        .filter(|&(x, y, z, _)| m_dist!(sx, sy, sz; x, y, z) <= r)
        .count()
}

fn part_2(input: Vec<(i32, i32, i32, i32)>) -> i32 {
    let (mut mx, mut nx) = input.iter().map(|&b| b.0).minmax().into_option().unwrap();
    let (mut my, mut ny) = input.iter().map(|&b| b.1).minmax().into_option().unwrap();
    let (mut mz, mut nz) = input.iter().map(|&b| b.2).minmax().into_option().unwrap();
    let mut s = nx - mx;
    let mut center = (i32::MAX, i32::MAX, i32::MAX);
    while s > 0 {
        center = iproduct!(
            (mx..=nx).step_by(s as usize),
            (my..=ny).step_by(s as usize),
            (mz..nz).step_by(s as usize)
        )
        .max_by_key(|&(x, y, z)| {
            input
                .iter()
                .filter(|&&(bx, by, bz, br)| m_dist!(x, y, z; bx, by, bz) - br < s)
                .count()
        })
        .unwrap();
        mx = center.0 - s;
        nx = center.0 + s;
        my = center.1 - s;
        ny = center.1 + s;
        mz = center.2 - s;
        nz = center.2 + s;
        s /= 2;
    }
    m_dist!(center.0, center.1, center.2)
}
