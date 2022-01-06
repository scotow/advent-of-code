advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<isize>) -> usize {
    solve(input, |r| *r += 1)
}

fn part_2(input: Vec<isize>) -> usize {
    solve(input, |r| {
        if *r >= 3 {
            *r -= 1;
        } else {
            *r += 1;
        }
    })
}

fn solve(mut input: Vec<isize>, ch: impl Fn(&mut isize)) -> usize {
    let mut n = 0;
    let mut i = 0;
    loop {
        match input.get_mut(i) {
            Some(r) => {
                i = (i as isize + *r) as usize;
                ch(r);
            }
            None => return n,
        }
        n += 1;
    }
}
