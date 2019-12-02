use std::io::{stdin, Read};
// use std::fs;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut program: Vec<u32> = input.split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    program[1] = 12;
    program[2] = 2;

    for i in (0..program.len() - 1).step_by(4) {
        println!("processing index: {}, value: {}", i, program[i]);
        match program[i] {
            1 => program[program[i + 3] as usize] = program[program[i + 1] as usize] + program[program[i + 2] as usize],
            2 => program[program[i + 3] as usize] = program[program[i + 1] as usize] * program[program[i + 2] as usize],
            99 => {
                println!("breaking at {}", i);
                break
            }
            _ => panic!("invalid instruction code")
        }

        println!("did set {}", program[i + 3]);
    }

    println!("Part 1: {}", program[0]);
}