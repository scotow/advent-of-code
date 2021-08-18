#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
pub fn part1(input: &usize) -> usize {
    for h in 1.. {
        let mut c = 0;
        for e in 1.. {
            if h % e == 0 {
                c += e* 10;
            }
            if c == *input {
                return h;
            }
            if c > *input || e > h {
                if h % 1000 == 0 {
                    dbg!(h);
                }
                break;
            }
        }
    }
    unreachable!();
}