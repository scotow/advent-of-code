advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<u8> {
    input.bytes().map(|n| n - b'0').collect()
}

fn part_1(input: Vec<u8>) -> usize {
    let layer = input
        .chunks(25 * 6)
        .min_by_key(|l| l.into_iter().filter(|&&n| n == 0).count())
        .unwrap();
    let counts = layer.into_iter().counts();
    counts[&1] * counts[&2]
}

fn part_2(input: Vec<u8>) -> String {
    (0..6)
        .map(|y| {
            (0..25)
                .map(|x| {
                    (input
                        .iter()
                        .skip(y * 25 + x)
                        .step_by(25 * 6)
                        .find(|&&p| p != 2)
                        .unwrap()
                        == &1)
                        .then(|| '#')
                        .unwrap_or(' ')
                })
                .collect::<String>()
        })
        .join("\n")
}
