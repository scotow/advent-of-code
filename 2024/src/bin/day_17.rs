advent_of_code_2024::main!();

fn generator(input: &str) -> ([u64; 3], Vec<u8>) {
    let (rgs, prog) = input.split_once("\n\n").unwrap();
    (
        rgs.lines().map(|l| l.split_once(": ").unwrap().1.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap(),
        prog.split(&[' ', ',']).filter_map(|n| n.parse().ok()).collect(),
    )
}

fn part_1((mut rgs, ops): ([u64; 3], Vec<u8>)) -> u64 {
    dbg!(&rgs, &ops);
    let mut ptr = 0;
    while ptr < ops.len() {
        // let prev_ptr = ptr;
        let arg = ops[ptr + 1];
        [adv, bxl, bst, jnz, bxc, out, bdv, cdv][ops[ptr] as usize](&mut ptr, &mut rgs, arg);
        // if ptr == prev_ptr {
        //     ptr += 2;
        // }
    }
    dbg!(&rgs);

    0
}

fn part_2((rgs, ops): ([u64; 3], Vec<u8>)) -> u64 {
    0
}

fn adv(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    rgs[0] = rgs[0] / 2u64.pow(combo(*rgs, arg) as u32);
    *ptr += 2;
}

fn bxl(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    rgs[1] = rgs[1] ^ arg as u64;
    *ptr += 2;
}

fn bst(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    rgs[1] = combo(*rgs, arg) % 8;
    *ptr += 2;
}

fn jnz(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    if rgs[0] == 0 {
        *ptr += 2;
    } else {
        *ptr = arg as usize;
    }
}

fn bxc(ptr: &mut usize, rgs: &mut [u64; 3], _arg: u8) {
    rgs[1] = rgs[1] ^ rgs[2];
    *ptr += 2;
}

fn out(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    print!("{},", combo(*rgs, arg) % 8);
    *ptr += 2;
}

fn bdv(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    rgs[1] = rgs[0] / 2u64.pow(combo(*rgs, arg) as u32);
    *ptr += 2;
}

fn cdv(ptr: &mut usize, rgs: &mut [u64; 3], arg: u8) {
    rgs[2] = rgs[0] / 2u64.pow(combo(*rgs, arg) as u32);
    *ptr += 2;
}

fn combo(rgs: [u64; 3], arg: u8) -> u64 {
    match arg {
        0..=3 => arg as u64,
        4..=6 => rgs[arg as usize - 4],
        _ => unreachable!(),
    }
}
