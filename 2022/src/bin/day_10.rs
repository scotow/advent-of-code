advent_of_code_2022::main!();

fn generator(input: &str) -> VecDeque<(usize, i32)> {
    input
        .lines()
        .map(|l| {
            let mut w = l.split_whitespace();
            match w.next().unwrap() {
                "noop" => (1, 0),
                "addx" => (2, w.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(ops: VecDeque<(usize, i32)>) -> i32 {
    let mut p = 0;
    run(ops, |c, r| {
        if c % 40 == 20 {
            p += c * r;
        }
    });
    p
}

fn part_2(ops: VecDeque<(usize, i32)>) -> String {
    let mut out = [false; 240];
    run(ops, |c, r| {
        out[c as usize - 1] = r.abs_diff((c - 1) % 40) <= 1;
    });
    out.chunks(40)
        .map(|l| l.iter().map(|p| p.then_some('#').unwrap_or(' ')).join(""))
        .join("\n")
}

fn run(mut ops: VecDeque<(usize, i32)>, mut f: impl FnMut(i32, i32)) {
    let mut r = 1;
    for c in 1..=240 {
        f(c, r);
        let Some((rem, _)) = ops.front_mut() else {
            break;
        };
        *rem -= 1;
        if *rem == 0 {
            r += ops.pop_front().unwrap().1;
        }
    }
}
