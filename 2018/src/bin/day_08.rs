advent_of_code_2018::main!();

#[derive(Debug, Clone)]
struct Node {
    sub: Vec<Node>,
    data: Vec<u16>,
}

impl Node {
    fn parse(bytes: &[u8], ptr: &mut usize) -> Self {
        let sub_count = bytes[*ptr];
        let data_count = bytes[*ptr + 1];
        *ptr += 2;
        Self {
            sub: (0..sub_count).map(|_| Self::parse(bytes, ptr)).collect(),
            data: (0..data_count)
                .map(|_| {
                    *ptr += 1;
                    bytes[*ptr - 1] as u16
                })
                .collect(),
        }
    }

    fn data(&self) -> u16 {
        self.data.iter().sum::<u16>() + self.sub.iter().map(|n| n.data()).sum::<u16>()
    }

    fn data_ref(&self) -> u16 {
        if self.sub.is_empty() {
            self.data()
        } else {
            self.data
                .iter()
                .map(|&i| {
                    self.sub
                        .get(i as usize - 1)
                        .map(Self::data_ref)
                        .unwrap_or(0)
                })
                .sum()
        }
    }
}

fn generator(input: &str) -> Node {
    Node::parse(
        &input
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect_vec(),
        &mut 0,
    )
}

fn part_1(root: Node) -> u16 {
    root.data()
}

fn part_2(root: Node) -> u16 {
    root.data_ref()
}
