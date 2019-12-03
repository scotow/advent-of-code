use std::io::{stdin, Read};

fn main() -> Result<(), &'static str> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let memory: Vec<u32> = input.split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let program = Program::new(memory);

    let part1 = program.compute(12, 2).unwrap();
    println!("Part 1: {}", part1);

    for noun in 0..100 {
        for verb in 0..100 {
            let part2 = program.compute(noun, verb).unwrap();
            if part2 == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                return Ok(())
            }
        }
    }

    Err("can't find a match for part2")
}

struct Program(Vec<u32>);

impl Program {
    fn new(mem: Vec<u32>) -> Program {
        Program(mem)
    }

    fn compute(&self, noun: u32, verb: u32) -> Result<u32, &str> {
        let Program(mem_origin) = self;
        let mut mem = mem_origin.clone();
        mem[1] = noun;
        mem[2] = verb;

        for i in (0..mem.len() - 1).step_by(4) {
            let target = mem[i + 3] as usize;
            match mem[i] {
                1 => mem[target] = mem[mem[i + 1] as usize] + mem[mem[i + 2] as usize],
                2 => mem[target] = mem[mem[i + 1] as usize] * mem[mem[i + 2] as usize],
                99 => break,
                _ => return Err("invalid instruction code")
            }
        }

        Ok(mem[0])
    }
}