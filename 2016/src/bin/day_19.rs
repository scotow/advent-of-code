advent_of_code_2016::main!();

fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

fn part_1(input: usize) -> usize {
    1 + (input - 2usize.pow((input as f64).log(2.) as u32)) * 2
}

fn part_2(input: usize) -> usize {
    let base = 3usize.pow((input as f64).log(3.) as u32);
    if input <= base * 2 {
        input - base
    } else {
        base + (input - base) * 2
    }
}
