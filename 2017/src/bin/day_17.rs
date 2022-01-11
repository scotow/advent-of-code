advent_of_code_2017::main!();

fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

fn part_1(input: usize) -> usize {
    let mut buffer = vec![0];
    let mut index = 0;
    for n in 1..=2017 {
        index = (index + input) % buffer.len() + 1;
        buffer.insert(index, n);
    }
    buffer[index + 1]
}

fn part_2(input: usize) -> usize {
    let (mut index, mut size, mut value_1) = (0, 1, 1);
    for n in 1..=50000000 {
        index = (index + input) % size + 1;
        if index == 1 {
            value_1 = n;
        }
        size += 1;
    }
    value_1
}
