use bitvec::field::BitField;
use bitvec::mem::BitMemory;
use bitvec::prelude::{BitSlice, BitVec, Msb0};

advent_of_code_2021::main!();

fn generator(input: &str) -> BitVec<Msb0, u8> {
    BitVec::from_vec(
        (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .collect(),
    )
}

fn part_1(input: BitVec<Msb0, u8>) -> u16 {
    Packet::new(input.as_bitslice(), &mut 0).version()
}

fn part_2(input: BitVec<Msb0, u8>) -> u64 {
    Packet::new(input.as_bitslice(), &mut 0).resolve()
}

#[derive(Debug)]
enum Packet {
    Literal(u16, u64),
    Operator(u16, u8, Vec<Packet>),
}

impl Packet {
    fn new(input: &BitSlice<Msb0, u8>, ptr: &mut usize) -> Self {
        fn read<N: BitMemory>(input: &BitSlice<Msb0, u8>, ptr: &mut usize, size: usize) -> N {
            *ptr += size;
            input[*ptr - size..*ptr].load_be()
        }

        let version = read(input, ptr, 3);
        let id = read(input, ptr, 3);
        if id == 4 {
            let mut n = 0;
            loop {
                let m = read::<u64>(input, ptr, 5);
                n = n << 4 | m & 0b01111;
                if m < 16 {
                    break;
                }
            }
            Packet::Literal(version, n)
        } else {
            let mut sub_packets = Vec::new();
            if read::<u8>(input, ptr, 1) == 0 {
                let length = read::<usize>(input, ptr, 15);
                let end = *ptr + length;
                while *ptr < end {
                    sub_packets.push(Self::new(input, ptr));
                }
            } else {
                let length = read::<usize>(input, ptr, 11);
                while sub_packets.len() < length {
                    sub_packets.push(Self::new(input, ptr));
                }
            }
            Packet::Operator(version, id, sub_packets)
        }
    }

    fn version(&self) -> u16 {
        match self {
            Packet::Literal(v, _) => *v,
            Packet::Operator(v, _, ss) => *v + ss.iter().map(Packet::version).sum::<u16>(),
        }
    }

    fn resolve(&self) -> u64 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Operator(_, id, ss) => match id {
                0 => ss.iter().map(Packet::resolve).sum(),
                1 => ss.iter().map(Packet::resolve).product(),
                2 => ss.iter().map(Packet::resolve).min().unwrap(),
                3 => ss.iter().map(Packet::resolve).max().unwrap(),
                5 => (ss[0].resolve() > ss[1].resolve()).into(),
                6 => (ss[0].resolve() < ss[1].resolve()).into(),
                _ => (ss[0].resolve() == ss[1].resolve()).into(),
            },
        }
    }
}
