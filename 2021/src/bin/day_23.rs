use pathfinding::prelude::dijkstra;

advent_of_code_2021::main!();

#[derive(Clone, Hash, Eq, PartialEq)]
struct Map {
    hall: [usize; 11],
    rooms: [Vec<usize>; 4],
    room_size: usize,
}

impl Map {
    fn next(&self) -> Vec<(Self, usize)> {
        let mut next = Vec::new();
        for (i, room) in self.rooms.iter().enumerate() {
            let owner = 10usize.pow(i as u32);
            if room.is_empty() || room.into_iter().all(|&f| f == owner) {
                continue;
            }
            let mut new = self.clone();
            let leaving = new.rooms[i].pop().unwrap();
            let distance = self.room_size - new.rooms[i].len();
            let out = 2 + 2 * i;
            for to_hall in (0..=1).chain((3..=7).step_by(2)).chain(9..=10) {
                if (out.min(to_hall)..=to_hall.max(out)).any(|h| self.hall[h] != 0) {
                    continue;
                }
                let mut new = new.clone();
                new.hall[to_hall] = leaving;
                next.push((
                    new,
                    (distance + out.max(to_hall) - to_hall.min(out)) * leaving,
                ));
            }
        }
        for (pos, &f) in self.hall.iter().enumerate().filter(|(_, &f)| f != 0) {
            let room_index = (f as f32).log10() as usize;
            if self.rooms[room_index].iter().any(|&of| of != f) {
                continue;
            }
            let room_pos = 2 + room_index * 2;
            if (pos.min(room_pos)..=pos.max(room_pos))
                .filter(|&p| p != pos)
                .any(|h| self.hall[h] != 0)
            {
                continue;
            }
            let mut distance = pos.max(room_pos) - pos.min(room_pos);
            distance += self.room_size - self.rooms[room_index].len();
            let mut new = self.clone();
            new.hall[pos] = 0;
            new.rooms[room_index].push(f);
            next.push((new, distance * f));
        }
        next
    }

    fn is_complete(&self) -> bool {
        self.rooms.iter().enumerate().all(|(i, room)| {
            room.len() == self.room_size && room.into_iter().all(|&f| f == 10usize.pow(i as u32))
        })
    }
}

fn generator(input: &str) -> Map {
    let (t, b) = input
        .lines()
        .skip(2)
        .take(2)
        .map(|l| {
            l.bytes()
                .filter(|&b| b >= b'A')
                .map(|c| 10usize.pow(3 - (b'D' - c) as u32))
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();
    Map {
        hall: [0; 11],
        rooms: [
            vec![b[0], t[0]],
            vec![b[1], t[1]],
            vec![b[2], t[2]],
            vec![b[3], t[3]],
        ],
        room_size: 2,
    }
}

fn part_1(input: Map) -> usize {
    dijkstra(&input, Map::next, Map::is_complete).unwrap().1
}

fn part_2(mut input: Map) -> usize {
    input.rooms[0].splice(1..1, [1000, 1000]);
    input.rooms[1].splice(1..1, [10, 100]);
    input.rooms[2].splice(1..1, [1, 10]);
    input.rooms[3].splice(1..1, [100, 1]);
    input.room_size = 4;
    dijkstra(&input, Map::next, Map::is_complete).unwrap().1
}
