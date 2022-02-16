advent_of_code_2019::main!();

fn generator(input: &str) -> [([i32; 3], [i32; 3]); 4] {
    input
        .lines()
        .map(|l| {
            (
                l.split(['=', ',', '>'])
                    .filter_map(|n| n.parse().ok())
                    .collect_vec()
                    .try_into()
                    .unwrap(),
                [0, 0, 0],
            )
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part_1(moons: [([i32; 3], [i32; 3]); 4]) -> i32 {
    (0..1000)
        .fold(moons, |prev, _| step(prev))
        .into_iter()
        .map(|(p, v)| {
            p.into_iter().map(|p| p.abs()).sum::<i32>()
                * v.into_iter().map(|v| v.abs()).sum::<i32>()
        })
        .sum()
}

fn part_2(moons: [([i32; 3], [i32; 3]); 4]) -> usize {
    (0..3)
        .map(|d| {
            successors(Some(moons), |&m| Some(step(m)))
                .map(|moons| {
                    <[i32; 8]>::try_from(
                        moons
                            .into_iter()
                            .flat_map(|(p, v)| [p[d], v[d]])
                            .collect_vec(),
                    )
                    .unwrap()
                })
                .enumerate()
                .duplicates_by(|&(_, ms)| ms)
                .next()
                .unwrap()
                .0
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}

fn step(moons: [([i32; 3], [i32; 3]); 4]) -> [([i32; 3], [i32; 3]); 4] {
    let mut next = moons;
    for (i, j, d) in iproduct!(0..4, 0..4, 0..3) {
        next[i].1[d] += (moons[j].0[d] - moons[i].0[d]).signum();
    }
    for ([x, y, z], [vx, vy, vz]) in &mut next {
        *x += *vx;
        *y += *vy;
        *z += *vz;
    }
    next
}
