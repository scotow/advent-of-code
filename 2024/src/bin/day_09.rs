advent_of_code_2024::main!();

fn generator(input: &str) -> Disk {
    Disk::parse(input)
}

fn part_1(mut disk: Disk) -> usize {
    loop {
        disk.head_next_hole();
        disk.move_tail();
        if disk.head >= disk.tail {
            break;
        }
        disk.sectors.swap(disk.head, disk.tail);
    }
    disk.checksum()
}

fn part_2(mut disk: Disk) -> usize {
    loop {
        match disk.last_chunk() {
            Some(chunk) => {
                if disk.head_to_first_hole(chunk.clone().count()) {
                    disk.sectors.copy_within(chunk.clone(), disk.head);
                    disk.sectors[chunk.clone()].fill(None);
                }
                disk.tail = *chunk.start() - 1;
                disk.move_tail();
            }
            None => break,
        }
    }
    disk.checksum()
}

#[derive(Clone)]
struct Disk {
    head: usize,
    tail: usize,
    sectors: Vec<Option<usize>>,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let sectors = input
            .bytes()
            .map(|b| b - b'0')
            .enumerate()
            .flat_map(|(i, d)| {
                if i % 2 == 0 {
                    repeat_n(Some(i / 2), d as usize)
                } else {
                    repeat_n(None, d as usize)
                }
            })
            .collect::<Vec<_>>();
        Self {
            head: 0,
            tail: sectors.len() - 1,
            sectors,
        }
    }

    fn head_next_hole(&mut self) {
        while self.sectors[self.head].is_some() {
            self.head += 1;
        }
    }

    fn head_to_first_hole(&mut self, size: usize) -> bool {
        match self.sectors[..self.tail]
            .windows(size)
            .enumerate()
            .find_map(|(i, ss)| ss.iter().all(|s| s.is_none()).then_some(i))
        {
            None => false,
            Some(pos) => {
                self.head = pos;
                true
            }
        }
    }

    fn move_tail(&mut self) {
        while self.sectors[self.tail].is_none() {
            self.tail -= 1;
        }
    }

    fn last_chunk(&self) -> Option<RangeInclusive<usize>> {
        self.sectors[..=self.tail]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, o)| (*o != self.sectors[self.tail]).then_some(i + 1..=self.tail))
    }

    fn checksum(self) -> usize {
        self.sectors
            .into_iter()
            .enumerate()
            .map(|(i, d)| i * d.unwrap_or(0))
            .sum()
    }
}
