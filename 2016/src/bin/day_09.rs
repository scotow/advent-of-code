advent_of_code_2016::main!();

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> usize {
    decompressed(input.as_bytes(), false)
}

fn part_2(input: &str) -> usize {
    decompressed(input.as_bytes(), true)
}

fn decompressed(input: &[u8], recursive: bool) -> usize {
    let mut count = 0;
    let mut index = 0;
    while index < input.len() {
        if input[index] == b'(' {
            index += 1;
            let marker_length = input[index..].iter().position(|&c| c == b')').unwrap();
            let marker = std::str::from_utf8(&input[index..index + marker_length]).unwrap();
            let (length, times) = marker
                .split('x')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            index += marker_length + 1;
            count += times
                * if recursive {
                    decompressed(&input[index..index + length], true)
                } else {
                    length
                };
            index += length;
        } else {
            index += 1;
            count += 1;
        }
    }
    count
}
