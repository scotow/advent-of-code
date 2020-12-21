use std::collections::HashMap;
use itertools::Itertools;
use itertools::iproduct;
use std::convert::TryInto;

const TILE_SIZE: usize = 10;
const IMAGE_SIZE: usize = 3;

type Data = [[bool; TILE_SIZE]; TILE_SIZE];
type Image = [[Data; IMAGE_SIZE]; IMAGE_SIZE];

#[aoc_generator(day20)]
fn input_generator(input: &str) -> HashMap<u16, Data> {
    input.split("\n\n")
        .map(|t| t.splitn(2, '\n').collect_tuple().unwrap())
        .map(|(t, d)| (
            t.replace("Tile ", "").replace(':', "").parse().unwrap(),
            d.lines()
                .map(|l| l.as_bytes().iter()
                    .map(|&c| c == b'#')
                    .collect_vec().try_into().unwrap()
                )
                .collect_vec().try_into().unwrap()
            )
        )
        .collect()
}

fn display(tile: &Data) -> String {
    tile.iter()
        .map(|r| r.iter()
            .map(|&c| if c { '#' } else { '.' }).collect::<String>())
        .join("\n")
}

fn possibilities(tile: &Data) -> Vec<Data> {
    (0..4)
        .scan(tile.clone(), |t, _| {
            let new = rotate(&t);
            *t = new;
            Some(vec![new, flip(&new)])
        })
        .flatten()
        .collect()
}

fn rotate(tile: &Data) -> Data {
    let mut rotated = Data::default();
    (0..TILE_SIZE).cartesian_product(0..TILE_SIZE)
        .for_each(|(x, y)| rotated[x][y] = tile[y][TILE_SIZE - x - 1]);
    rotated
}

fn flip(tile: &Data) -> Data {
    let mut flipped = tile.clone();
    flipped.iter_mut()
        .for_each(|r| r.reverse());
    flipped
}

fn line_up(image: &Image) -> bool {
    image.iter()
        .all(|r| r.iter()
            .tuple_windows()
            .all(|(t1, t2)| {
                (0..TILE_SIZE).all(|y| t1[y][TILE_SIZE - 1] == t2[y][0])
            })
        )
    &&
        image.iter()
            .tuple_windows()
            .all(|(r1, r2)| {
                (0..IMAGE_SIZE).all(|t| {
                    (0..TILE_SIZE).all(|x| r1[t][TILE_SIZE - 1][x] == r2[t][0][x])
                })
            })
}

fn build_image(data: &[Data]) -> Image {
    dbg!(data.len());
    data.chunks(IMAGE_SIZE)
        .map(|r| r.to_vec().try_into().unwrap())
        .collect_vec().try_into().unwrap()
}

#[aoc(day20, part1)]
fn part1(input: &HashMap<u16, Data>) -> u16 {
    let mut cache = input.iter()
        .map(|(k, t)| (k, possibilities(t)))
        .collect::<HashMap<_, _>>();

    let image = input.keys()
        .permutations(input.len())
        .find(|ts| {
            dbg!(ts.len());
            let image = ts.iter()
                .cartesian_product(0..8)
                .map(|(k, v)| cache[k][v])
                .collect_vec();
            dbg!(image.len());
            let image = build_image(&image);
            line_up(&image)
        });

    dbg!(image);

    // dbg!((0..IMAGE_SIZE).permutations(IMAGE_SIZE).collect_vec());

    // println!("{}",
    //     possibilities(&input[&1]).iter()
    //         .map(|t| display(t))
    //         .join("\n\n")
    //     );
    0
}