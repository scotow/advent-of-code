use itertools::iproduct;
use itertools::Itertools;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(usize, usize, u8)> {
    input.lines()
        .map(|l| l.rsplitn(4, ' ').flat_map(|l| l.split(',')).collect_vec())
        .map(|l| (match l[5] {
            "turn on" => 0,
            "turn off" => 1,
            "toggle" => 2,
            _ => unreachable!(),
        }, l))
        .flat_map(|(a, l)| {
            iproduct!(
                l[3].parse::<usize>().unwrap()..=l[0].parse::<usize>().unwrap(),
                l[4].parse::<usize>().unwrap()..=l[1].parse::<usize>().unwrap()
            ).map(move |(x, y)| (x, y, a))
        })
        .collect()
}

fn turn_on(l: &mut u32) { *l |= 1 }
fn turn_off(l: &mut u32) { *l &= 0 }
fn toggle(l: &mut u32) { *l ^= 1 }

fn increase(l: &mut u32) { *l += 1 }
fn decrease(l: &mut u32) { *l = l.saturating_sub(1) }
fn increase2(l: &mut u32) { *l += 2 }

#[aoc(day6, part1)]
pub fn part1(input: &Vec<(usize, usize, u8)>) -> u32 {
    solve(input, |a| match a {
        0 => turn_on,
        1 => turn_off,
        2 => toggle,
        _ => unreachable!(),
    })
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<(usize, usize, u8)>) -> u32 {
    solve(input, |a| match a {
        0 => increase,
        1 => decrease,
        2 => increase2,
        _ => unreachable!(),
    })
}

fn solve<F>(input: &Vec<(usize, usize, u8)>, matcher: F) -> u32
where F: Fn(u8) -> fn(&mut u32)
{
    let mut grid = [[0u32; 1000]; 1000];
    input.iter()
        .map(|(x, y, a)| (x, y, matcher(*a)))
        .for_each(|(&x, &y, a)| {
            a(&mut grid[y][x]);
        });

    grid.iter()
        .flat_map(|r| r.iter())
        .sum()
}
