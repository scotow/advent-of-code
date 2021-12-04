advent_of_code_2021::main!();

fn generator(input: &str) -> (usize, Vec<u32>) {
    (
        input.lines().next().unwrap().len(),
        input
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect(),
    )
}

fn part_1((length, inputs): (usize, Vec<u32>)) -> u32 {
    let gamma = (0..length)
        .rev()
        .map(|i| {
            inputs
                .iter()
                .map(|n| n >> i & 1)
                .counts()
                .into_iter()
                .max_by_key(|&(_, c)| c)
                .unwrap()
                .0
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
