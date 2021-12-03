advent_of_code_2021::main!();

fn generator(input: &str) -> (usize, Vec<u32>) {
    (
        input.lines().next().unwrap().len(),
        input
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect()
    )
}

fn part_1((length, inputs): (usize, Vec<u32>)) -> u32 {
    let gamma = (0..length).rev()
        .map(|i| {
            let (z, o) = inputs
                .iter()
                .fold((0, 0), |(z, o), n| {
                    if n >> i & 1 == 0 { (z + 1, o) } else { (z, o + 1) }
                });
            if z > o { 0 } else { 1 }
        })
        .fold(0, |acc, b| (acc << 1) | b);
    gamma * (gamma ^ !(!0 << length))
}

fn part_2((length, inputs): (usize, Vec<u32>)) -> u32 {
    let find = |cmp: fn(&usize, &usize) -> bool| {
        let mut inputs = inputs.clone();
        let mut i = length;
        while inputs.len() > 1 {
            i -= 1;
            let (zs, os) = inputs
                .into_iter()
                .partition::<Vec<_>, _>(|n| n >> i & 1 == 0);
            inputs = if cmp(&zs.len(), &os.len()) { zs } else { os }
        }
        inputs[0]
    };
    find(usize::gt) * find(usize::le)
}
