advent_of_code_2017::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> u16 {
    let mut numbers = (0..=255).collect_vec();
    solve(
        &mut numbers,
        &input.split(',').map(|s| s.parse().unwrap()).collect_vec(),
        &mut 0,
        &mut 0,
    );
    numbers[0] * numbers[1]
}

fn part_2(input: &str) -> String {
    let mut numbers = (0..=255).collect_vec();
    let input = input
        .bytes()
        .map(|c| c as usize)
        .chain([17, 31, 73, 47, 23])
        .collect_vec();
    let mut ptr = 0;
    let mut skip = 0;
    for _ in 0..64 {
        solve(&mut numbers, &input, &mut ptr, &mut skip);
    }
    numbers
        .chunks(16)
        .map(|chunk| {
            format!(
                "{:02x}",
                chunk.into_iter().copied().reduce(|acc, n| acc ^ n).unwrap()
            )
        })
        .join("")
}

fn solve(numbers: &mut [u16], lens: &[usize], ptr: &mut usize, skip: &mut usize) {
    let numbers_len = numbers.len();
    for &len in lens {
        let to_rev = numbers
            .iter()
            .cycle()
            .skip(*ptr)
            .take(len)
            .copied()
            .collect_vec();
        for (i, r_i) in (*ptr..*ptr + len)
            .map(|i| i % numbers_len)
            .rev()
            .enumerate()
        {
            numbers[r_i] = to_rev[i];
        }
        *ptr += len + *skip;
        *skip += 1;
    }
}
