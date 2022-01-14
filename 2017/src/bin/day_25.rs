advent_of_code_2017::main!();

type Command = (bool, isize, usize);

fn generator(input: &str) -> (usize, usize, Vec<[Command; 2]>) {
    fn parse_command<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Command {
        iter.next();
        (
            iter.next().unwrap().ends_with("1."),
            if iter.next().unwrap().ends_with("right.") {
                1
            } else {
                -1
            },
            (iter.next().unwrap().as_bytes()[22] - b'A') as usize,
        )
    }

    let mut parts = input.split("\n\n");
    let (start, steps) = parts.next().unwrap().lines().collect_tuple().unwrap();
    (
        (start.as_bytes()[start.len() - 2] - b'A') as usize,
        steps.split_whitespace().nth(5).unwrap().parse().unwrap(),
        parts
            .map(|c| {
                let mut iter = c.lines().skip(1).map(|l| l.trim());
                [parse_command(&mut iter), parse_command(&mut iter)]
            })
            .collect(),
    )
}

fn part_1((mut state, count, commands): (usize, usize, Vec<[Command; 2]>)) -> usize {
    let mut tape = HashMap::new();
    let mut ptr = 0;
    for _ in 0..count {
        let (val, offset, next) = commands[state][*tape.get(&ptr).unwrap_or(&false) as usize];
        tape.insert(ptr, val);
        ptr += offset;
        state = next;
    }
    tape.into_values().filter(|&v| v).count()
}

fn part_2(_input: (usize, usize, Vec<[Command; 2]>)) -> &'static str {
    "N/A"
}
