use bitvec::field::BitField;
use bitvec::macros::internal::funty::Integral;
// use bitvec::mem::BitMemory;
use bitvec::prelude::{BitSlice, BitVec, Msb0};

advent_of_code_2021::main!();

fn generator(input: &str) -> BitVec<u8, Msb0> {
    BitVec::from_vec(
        (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .collect(),
    )
}

fn part_1(input: BitVec<u8, Msb0>) -> u16 {
    parse(input.as_bitslice(), &mut 0).0
}

fn part_2(input: BitVec<u8, Msb0>) -> u64 {
    parse(input.as_bitslice(), &mut 0).1
}

fn parse(input: &BitSlice<u8, Msb0>, ptr: &mut usize) -> (u16, u64) {
    let mut version = read(input, ptr, 3);
    let n = match read::<u8>(input, ptr, 3) {
        4 => {
            let mut n = 0;
            loop {
                let m = read::<u64>(input, ptr, 5);
                n = n << 4 | m & 0b01111;
                if m < 16 {
                    break;
                }
            }
            n
        }
        id => {
            let mut ss = Vec::new();
            if read::<u8>(input, ptr, 1) == 0 {
                let length = read::<usize>(input, ptr, 15);
                let end = *ptr + length;
                while *ptr < end {
                    let (v, n) = parse(input, ptr);
                    version += v;
                    ss.push(n);
                }
            } else {
                let length = read::<usize>(input, ptr, 11);
                while ss.len() < length {
                    let (v, n) = parse(input, ptr);
                    version += v;
                    ss.push(n);
                }
            }
            match id {
                0 => ss.into_iter().sum(),
                1 => ss.into_iter().product(),
                2 => ss.into_iter().min().unwrap(),
                3 => ss.into_iter().max().unwrap(),
                5 => (ss[0] > ss[1]).into(),
                6 => (ss[0] < ss[1]).into(),
                _ => (ss[0] == ss[1]).into(),
            }
        }
    };
    (version, n)
}

fn read<N: Integral>(input: &BitSlice<u8, Msb0>, ptr: &mut usize, size: usize) -> N {
    *ptr += size;
    input[*ptr - size..*ptr].load_be()
}
