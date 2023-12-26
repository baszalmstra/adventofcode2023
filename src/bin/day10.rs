use itertools::Either;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [
            Direction::West,
            Direction::South,
            Direction::East,
            Direction::North,
        ]
        .iter()
        .copied()
    }

    pub fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

fn main() {
    let input = std::fs::read_to_string("inputs/day10/input.txt").unwrap();

    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Tile::Pipe(Direction::North, Direction::South),
                    '-' => Tile::Pipe(Direction::East, Direction::West),
                    'L' => Tile::Pipe(Direction::North, Direction::East),
                    'J' => Tile::Pipe(Direction::North, Direction::West),
                    '7' => Tile::Pipe(Direction::South, Direction::West),
                    'F' => Tile::Pipe(Direction::South, Direction::East),
                    'S' => Tile::Start,
                    '.' => Tile::Ground,
                    _ => panic!("Unknown tile: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let starting_pos = lines
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| match tile {
                Tile::Start => Some((x, y)),
                _ => None,
            })
        })
        .unwrap();

    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    queue.push_back((starting_pos, 0));
    seen.insert(starting_pos, 0);

    while let Some((pos, distance)) = queue.pop_front() {
        let tile = &lines[pos.1][pos.0];
        let directions = match tile {
            Tile::Pipe(a, b) => Either::Left([a, b].into_iter().copied()),
            Tile::Ground => continue,
            Tile::Start => Either::Right(Direction::all()),
        };

        for direction in directions {
            let delta = direction.delta();
            let new_pos = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if seen.contains_key(&new_pos) {
                continue;
            }
            let Some(Tile::Pipe(a, b)) = lines.get(new_pos.1).and_then(|row| row.get(new_pos.0))
            else {
                continue;
            };

            if *a != direction.opposite() && *b != direction.opposite() {
                continue;
            }

            seen.insert(new_pos, distance + 1);
            queue.push_back((new_pos, distance + 1));
        }
    }

    let furthest = *seen.values().max().unwrap();
    println!("Solution 1: {}", furthest);

    let mut is_loop = HashSet::new();
    let mut inside_loop = HashSet::new();
    let mut pos = starting_pos;
    loop {
        is_loop.insert(pos);

        let tile = &lines[pos.1][pos.0];
        let mut directions = match tile {
            Tile::Pipe(a, b) => Either::Left([a, b].into_iter().copied()),
            Tile::Start => Either::Right(Direction::all()),
            Tile::Ground => panic!("Unexpected ground tile"),
        };

        let Some((next_pos, from_dir, next_dir)) = directions.find_map(|direction| {
            let delta = direction.delta();
            let new_pos = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                return None;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if is_loop.contains(&new_pos) {
                return None;
            }
            let Some(Tile::Pipe(a, b)) = lines.get(new_pos.1).and_then(|row| row.get(new_pos.0))
            else {
                return None;
            };
            let other_dir = if *a == direction.opposite() {
                *b
            } else if *b == direction.opposite() {
                *a
            } else {
                return None;
            };
            Some((new_pos, direction, other_dir))
        }) else {
            break;
        };

        for dir in [from_dir.left(), next_dir.left()] {
            let delta = dir.delta();
            let new_pos = (next_pos.0 as isize + delta.0, next_pos.1 as isize + delta.1);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if seen.contains_key(&new_pos) {
                continue;
            }

            inside_loop.insert(new_pos);
        }

        pos = next_pos;
    }

    let mut queue = VecDeque::from_iter(inside_loop.iter().copied());
    while let Some(next) = queue.pop_front() {
        for dir in Direction::all() {
            let delta = dir.delta();
            let new_pos = (next.0 as isize + delta.0, next.1 as isize + delta.1);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if inside_loop.contains(&new_pos) || is_loop.contains(&new_pos) {
                continue;
            }

            inside_loop.insert(new_pos);
            queue.push_back(new_pos);
        }
    }

    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            let pos = (x, y);
            if inside_loop.contains(&pos) {
                print!("I");
            } else if is_loop.contains(&pos) {
                print!("O")
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("Solution 2: {}", inside_loop.len());
}
