advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<u32>) -> u32 {
    input.into_iter().map(fuel).sum()
}

fn part_2(input: Vec<u32>) -> u32 {
    input.into_iter().map(r_fuel).sum()
}

fn fuel(n: u32) -> u32 {
    n / 3 - 2
}

fn r_fuel(n: u32) -> u32 {
    let mut f = fuel(n);
    if f > 8 {
        f += r_fuel(f);
    }
    f
}
