advent_of_code_2023::main!();

fn generator(input: &str) -> impl Iterator<Item = &str> + Clone {
    input.split(',')
}

fn part_1(input: impl Iterator<Item = &'static str>) -> usize {
    input.map(hash).sum()
}

fn part_2(input: impl Iterator<Item = &'static str>) -> usize {
    let mut boxes = vec![Vec::new(); 256];
    for lens in input {
        let (b, n) = lens.split_once(['-', '=']).unwrap();
        let target = &mut boxes[hash(b)];
        let i = target.iter().position(|&(l, _)| l == b);
        if n.is_empty() {
            i.map(|i| target.remove(i));
        } else {
            let n = n.parse::<usize>().unwrap();
            match i {
                Some(i) => target[i] = (b, n),
                None => target.push((b, n)),
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            (i + 1)
                * b.into_iter()
                    .enumerate()
                    .map(|(i, (_, n))| (i + 1) * n)
                    .sum::<usize>()
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |a, b| (a + b as usize) * 17 % 256)
}
