advent_of_code_2017::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> u16 {
    let mut numbers = (0..=255).collect_vec().try_into().unwrap();
    knot_hash::knot(
        &mut numbers,
        &input.split(',').map(|s| s.parse().unwrap()).collect_vec(),
        &mut 0,
        &mut 0,
    );
    numbers[0] as u16 * numbers[1] as u16
}

fn part_2(input: &str) -> String {
    knot_hash::hash(input)
        .map(|n| format!("{:02x}", n))
        .join("")
}
