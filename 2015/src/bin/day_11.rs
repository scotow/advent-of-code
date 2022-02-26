advent_of_code_2015::main!();

type Password = [u8; 8];

fn generator(input: &str) -> Password {
    input.as_bytes().try_into().unwrap()
}

fn part_1(input: Password) -> String {
    let mut pass = input.clone();
    next(&mut pass);
    std::str::from_utf8(&pass).unwrap().to_string()
}

fn part_2(input: Password) -> String {
    let mut pass = input.clone();
    next(&mut pass);
    increment(&mut pass);
    next(&mut pass);
    std::str::from_utf8(&pass).unwrap().to_string()
}

fn next(pass: &mut Password) {
    while !is_valid(&pass) {
        increment(pass);
    }
}

fn is_valid(pass: &Password) -> bool {
    pass.iter()
        .tuple_windows()
        .any(|(a, b, c)| a + 1 == *b && b + 1 == *c)
        && pass.iter().all(|&c| c != b'i' && c != b'o' && c != b'l')
        && pass
            .iter()
            .tuple_windows()
            .filter(|(a, b)| a == b)
            .sorted()
            .dedup()
            .count()
            >= 2
}

fn increment(pass: &mut [u8]) {
    if pass.len() > 1 && *pass.last().unwrap() == b'z' {
        let to = pass.len() - 1;
        increment(&mut pass[..to])
    }
    *pass.last_mut().unwrap() = (pass.last().unwrap() + 1 - b'a') % 26 + b'a';
}
