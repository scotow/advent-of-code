use itertools::Itertools;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    decompress(input.as_bytes()).len()
}

fn decompress(input: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut index = 0;
    while index < input.len() {
        if input[index] == b'(' {
            index += 1;

            let marker_length = input[index..].iter().position(|&c| c == b')').unwrap();
            let marker = std::str::from_utf8(&input[index..index + marker_length]).unwrap();
            let (length, times) = marker.split('x').map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap();
            index += marker_length + 1;

            let copy = &input[index..index + length];
            for _ in 0..times {
                decompressed.extend_from_slice(copy);
            }
            index += length;
        } else {
            decompressed.push(input[index]);
            index += 1;
        }
    }
    decompressed
}