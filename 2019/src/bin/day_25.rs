advent_of_code_2019::main!();

#[derive(Clone, Debug)]
struct Room {
    doors: Vec<String>,
    item: Option<String>,
    prog: Program,
}

impl Room {
    fn parse(prog: &mut Program) -> Self {
        let info = prog
            .pull_all()
            .into_iter()
            .map(|c| c as u8 as char)
            .collect::<String>();
        let doors_pos = info.lines().position(|l| l == "Doors here lead:").unwrap();
        let doors = info
            .lines()
            .skip(doors_pos + 1)
            .take_while(|l| l.starts_with('-'))
            .map(|l| l.trim_start_matches("- ").to_owned())
            .collect_vec();

        let item = if let Some(items_pos) = info.lines().position(|l| l == "Items here:") {
            Some(
                info.lines()
                    .nth(items_pos + 1)
                    .unwrap()
                    .trim_start_matches("- ")
                    .to_owned(),
            )
        } else {
            None
        };

        // println!("{}", &info);

        Self {
            doors,
            item,
            prog: prog.clone(),
        }
    }
}

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i64 {
    prog.run();
    let mut grid = HashMap::from([((0, 0), Room::parse(&mut prog))]);
    loop {
        let mut next = grid.clone();
        let prev_len = grid.len();
        for ((x, y), room) in grid {
            for door in room.doors {
                // dbg!(door);
                let pos = match door.as_ref() {
                    "north" => (x, y - 1),
                    "east" => (x + 1, y),
                    "south" => (x, y + 1),
                    "west" => (x - 1, y),
                    _ => unreachable!(),
                };
                if next.contains_key(&pos) {
                    continue;
                }
                let mut prog = room.prog.clone();
                prog.push_multiple(door.bytes().chain(once(b'\n')).map_into());
                prog.run();
                next.insert(pos, Room::parse(&mut prog));
            }
        }
        grid = next;
        if grid.len() == prev_len {
            break;
        }
    }
    dbg!(&grid[&(0, 1)]);
    // dbg!(grid.values().filter_map(|r| r.item.as_ref()).collect_vec());

    0
}

fn part_2(_: Program) -> &'static str {
    "N/A"
}
