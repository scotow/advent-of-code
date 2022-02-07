use pathfinding::prelude::dijkstra_all;

advent_of_code_2018::main!();

type Map = HashMap<(i16, i16), HashSet<(i16, i16)>>;

fn generator(input: &str) -> Map {
    let mut map = HashMap::new();
    parse(
        input[1..input.len() - 1].as_bytes(),
        &mut 0,
        &mut map,
        (0, 0),
    );
    map
}

fn parse(rgx: &[u8], ptr: &mut usize, map: &mut Map, start: (i16, i16)) -> (i16, i16) {
    let mut pos = start;
    while *ptr < rgx.len() {
        *ptr += 1;
        let delta = match rgx[*ptr - 1] {
            b'N' => Some((0, -1)),
            b'E' => Some((1, 0)),
            b'S' => Some((0, 1)),
            b'W' => Some((-1, 0)),
            b'(' => {
                pos = parse(rgx, ptr, map, pos);
                None
            }
            b'|' => {
                parse(rgx, ptr, map, start);
                return pos;
            }
            b')' => return pos,
            _ => unreachable!(),
        };
        if let Some((dx, dy)) = delta {
            map.entry(pos).or_default().insert((dx, dy));
            pos.0 += dx;
            pos.1 += dy;
        }
    }
    (0, 0)
}

fn part_1(input: Map) -> usize {
    solve(input).max().unwrap()
}

fn part_2(input: Map) -> usize {
    solve(input).filter(|&n| n >= 1000).count()
}

fn solve(map: Map) -> impl Iterator<Item = usize> {
    dijkstra_all(&(0, 0), |&(x, y)| {
        map.get(&(x, y))
            .map(|ngs| {
                ngs.into_iter()
                    .map(|&(dx, dy)| ((x + dx, y + dy), 1))
                    .collect_vec()
            })
            .unwrap_or_default()
    })
    .into_values()
    .map(|(_, n)| n)
}
