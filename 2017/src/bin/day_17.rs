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
    let mut buffer = Vec::with_capacity(50000000);
    buffer.push(0);
    let mut index = 0;
    for n in 1usize..=50000000 {
        index = (index + input) % buffer.len() + 1;
        buffer.insert(index, n);
        if n % 100000 == 0 {
            dbg!(n);
        }
    }
    buffer
        .into_iter()
        .tuple_windows()
        .find(|&(n1, _)| n1 == 0)
        .unwrap()
        .1
}
