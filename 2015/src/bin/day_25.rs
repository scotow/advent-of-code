advent_of_code_2015::main!();

fn generator(input: &str) -> (usize, usize) {
    let mut parts = input.split(' ');
    (
        parts
            .nth(16)
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse()
            .unwrap(),
        parts
            .nth(1)
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .parse()
            .unwrap(),
    )
}

fn part_1(input: (usize, usize)) -> u64 {
    let (mut x, mut y, mut n) = (6, 6, 27995004);
    while x != input.1 || y != input.0 {
        if y == 1 {
            y = x + 1;
            x = 1;
        } else {
            y -= 1;
            x += 1;
        }
        n = (n * 252533) % 33554393;
    }
    n
}

fn part_2(_: (usize, usize)) -> &'static str {
    "N/A"
}
