advent_of_code_2017::main!();

fn generator(input: &str) -> &[u8] {
    input.as_bytes()
}

fn part_1(input: &[u8]) -> usize {
    parse(input, &mut 0, 1).0
}

fn part_2(input: &[u8]) -> usize {
    parse(input, &mut 0, 1).1
}

fn parse(input: &[u8], ptr: &mut usize, depth: usize) -> (usize, usize) {
    *ptr += 1;
    let mut groups = depth;
    let mut comments = 0;
    loop {
        match input[*ptr] {
            b'{' => {
                let (g, c) = parse(input, ptr, depth + 1);
                groups += g;
                comments += c;
            }
            b',' => *ptr += 1,
            b'<' => {
                *ptr += 1;
                while input[*ptr] != b'>' {
                    if input[*ptr] == b'!' {
                        *ptr += 2;
                    } else {
                        *ptr += 1;
                        comments += 1;
                    }
                }
                *ptr += 1;
            }
            b'}' => {
                *ptr += 1;
                return (groups, comments);
            }
            _ => unreachable!(),
        }
    }
}
