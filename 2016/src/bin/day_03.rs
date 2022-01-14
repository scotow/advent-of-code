advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<[u16; 3]> {
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|n| n.parse::<u16>().ok())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn part_1(triangles: Vec<[u16; 3]>) -> usize {
    valids(triangles)
}

fn part_2(input: Vec<[u16; 3]>) -> usize {
    let mut triangles = vec![[0, 0, 0]; input.len()];
    for (i, r) in input.iter().enumerate() {
        triangles[i / 3 * 3][i % 3] = r[0];
        triangles[i / 3 * 3 + 1][i % 3] = r[1];
        triangles[i / 3 * 3 + 2][i % 3] = r[2];
    }
    valids(triangles)
}

fn valids(triangles: Vec<[u16; 3]>) -> usize {
    triangles
        .iter()
        .map(|pts| {
            let mut pts = pts.clone();
            pts.sort();
            pts
        })
        .filter(|&[a, b, c]| a + b > c)
        .count()
}
