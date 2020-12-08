use itertools::Itertools;
use std::collections::HashSet;
use Action::*;
use AlgoResult::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Acc,
    Jmp,
    Nop,
}

impl Action {

    fn to_fn(&self) -> fn(&mut usize, &mut i32, i32) {
        match self {
            Acc => acc,
            Jmp => jmp,
            Nop => nop,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(Action, i32)> {
    input.lines()
        .map(|l| l.split(' ').collect_tuple().unwrap())
        .map(|(a, p)| (match a {
            "acc" => Acc,
            "jmp" => Action::Jmp,
            "nop" => Action::Nop,
            _ => unreachable!(),
        }, p.parse::<i32>().unwrap()))
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Vec<(Action, i32)>) -> i32 {
    match solve(input) {
        Blocked(r) => r,
        Terminated(_) => unreachable!(),
    }
}

#[aoc(day8, part2)]
fn part2(input: &Vec<(Action, i32)>) -> i32 {
    input.iter()
        .enumerate()
        .find_map(|(i, (a, p))| {
            let mut copy = input.clone();
            copy[i] = (match a {
                Acc => Acc,
                Jmp => Nop,
                Nop => Jmp,
            }, *p);

            match solve(&copy) {
                Blocked(_) => None,
                Terminated(r) => Some(r),
            }
        })
        .unwrap()
}

enum AlgoResult<T> {
    Blocked(T),
    Terminated(T),
}

fn solve(input: &Vec<(Action, i32)>) -> AlgoResult<i32> {
    let mut visited = HashSet::<usize>::new();
    let mut pointer = 0;
    let mut accumulator = 0;

    loop {
        if pointer == input.len() {
            return Terminated(accumulator)
        }
        if visited.contains(&pointer) {
            return Blocked(accumulator);
        }
        visited.insert(pointer);

        let (act, param) = input[pointer];
        act.to_fn()(&mut pointer, &mut accumulator, param);
    }
}

fn acc(pointer: &mut usize, acc: &mut i32, param: i32) {
    *acc += param;
    *pointer += 1;
}

fn jmp(pointer: &mut usize, _acc: &mut i32, param: i32) {
    *pointer = ((*pointer as i32) + param) as usize
}

fn nop(pointer: &mut usize, _acc: &mut i32, _param: i32) {
    *pointer += 1;
}