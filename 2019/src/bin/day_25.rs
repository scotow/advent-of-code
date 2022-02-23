advent_of_code_2019::main!();

#[derive(Clone, Debug)]
struct Room {
    doors: Vec<String>,
    item: Option<String>,
    prog: Program,
}

impl Room {
    fn parse(prog: &mut Program) -> (Self, String) {
        let info = prog
            .pull_all()
            .into_iter()
            .map(|c| c as u8 as char)
            .skip_while(|c| c.is_whitespace())
            .collect::<String>();

        let name = info
            .lines()
            .next()
            .unwrap()
            .trim_matches(['=', ' '].as_slice())
            .to_owned();

        // println!("{}", &info);

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

        (
            Self {
                doors,
                item,
                prog: prog.clone(),
            },
            name,
        )
    }
}

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i64 {
    prog.run();
    let (start_room, start_name) = Room::parse(&mut prog);
    let mut map = HashMap::from([(start_name, (start_room, Vec::new()))]);
    loop {
        let mut next = map.clone();
        for (name, (room, path)) in map.iter() {
            if name == "Pressure-Sensitive Floor" {
                continue;
            }
            for door in &room.doors {
                let mut prog = room.prog.clone();
                prog.push_multiple(door.bytes().chain(once(b'\n')).map_into());
                prog.run();
                let mut path = path.clone();
                path.push(door.clone());
                let (room, name) = Room::parse(&mut prog);
                if !next.contains_key(&name) {
                    next.insert(name, (room, path));
                }
            }
        }
        if next.len() == map.len() {
            break;
        }
        map = next;
    }

    for (room, path) in map.values() {
        if room
            .item
            .as_ref()
            .map(|item| {
                [
                    "photons",
                    "escape pod",
                    "molten lava",
                    "infinite loop",
                    "giant electromagnet",
                ]
                .contains(&item.as_ref())
            })
            .unwrap_or(true)
        {
            continue;
        }
        for door in path {
            prog.push_multiple(door.bytes().chain(once(b'\n')).map_into());
        }
        prog.push_multiple(
            chain!(
                "take ".bytes(),
                room.item.as_ref().unwrap().bytes(),
                once(b'\n')
            )
            .map_into(),
        );
        for door in reverse_path(path) {
            prog.push_multiple(door.bytes().chain(once(b'\n')).map_into());
        }

        // dbg!(&room.item);
    }

    prog.push_multiple("inv\n".bytes().map_into());

    prog.run();
    println!(
        "{}",
        prog.pull_all()
            .into_iter()
            .map(|c| c as u8 as char)
            .skip_while(|c| c.is_whitespace())
            .collect::<String>()
    );

    // dbg!(map.values().filter_map(|r| r.0.item.as_ref()).collect_vec());
    // dbg!(&map["Security Checkpoint"]);
    // dbg!(reverse_path(&map["Security Checkpoint"].1));

    0
}

fn part_2(_: Program) -> &'static str {
    "N/A"
}

fn reverse_path(path: &[String]) -> Vec<String> {
    path.into_iter()
        .rev()
        .map(|d| {
            match d.as_ref() {
                "north" => "south",
                "east" => "west",
                "south" => "north",
                "west" => "east",
                _ => unreachable!(),
            }
            .to_owned()
        })
        .collect()
}
