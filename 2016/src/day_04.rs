use itertools::Itertools;

#[derive(Debug)]
pub struct Room {
    name: Vec<u8>,
    sector: u32,
    checksum: Vec<u8>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Room> {
    input.lines()
        .map(|l| {
            let (name, left) = l.rsplit_once('-').unwrap();
            let (sector, checksum) = left[..left.len() - 1].split_once('[').unwrap();
            Room {
                name: name.replace('-', "").as_bytes().to_vec(),
                sector: sector.parse().unwrap(),
                checksum: checksum.as_bytes().to_vec(),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(rooms: &Vec<Room>) -> u32 {
    rooms.iter()
        .filter(|r| {
            let mut counts = r.name.iter().counts().into_iter()
                .collect_vec();
            counts.sort_by(|(c1, n1), (c2, n2)| n1.cmp(n2).reverse().then(c1.cmp(c2)));
            counts.iter()
                .map(|(c, _)| *c)
                .take(r.checksum.len())
                .eq(r.checksum.iter())
        })
        .map(|r| r.sector)
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(rooms: &Vec<Room>) -> u32 {
    rooms.iter()
        .map(|r| {
            (
                String::from_utf8(
                    r.name.iter()
                        .map(|&c| ((c as u32 - 97 + r.sector) % 26 + 97) as u8)
                        .collect(),
                ).unwrap(),
                r.sector
            )
        })
        .find_map(|(name, sector)| (name == "northpoleobjectstorage").then(|| sector))
        .unwrap()
}