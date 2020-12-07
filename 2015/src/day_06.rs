use itertools::Itertools;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<((usize, usize, fn(&mut u8)))> {
    input.lines()
        .map(|l| l.rsplitn(4, ' ').flat_map(|l| l.split(',')).collect_vec())
        .map(|l| (/*match l[5] {
            "turn on" => turn_on,
            "turn off" => turn_off,
            "toggle" => toggle,
            _ => unreachable!()
        }*/ (l[3].parse::<usize>().unwrap()..l[0].parse::<usize>().unwrap()).flat_map(|r| )))
        .for_each(|l| println!("{:?}", l));

    vec![]
}

fn turn_on(l: &mut u8) { *l |= 1 }
fn turn_off(l: &mut u8) { *l &= 1 }
fn toggle(l: &mut u8) { *l ^= 1 }

#[aoc(day6, part1)]
pub fn part1(input: &Vec<((usize, usize, fn(&mut u8)))>) -> usize {
    0
}
