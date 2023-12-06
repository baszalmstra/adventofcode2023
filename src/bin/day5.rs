use itertools::Itertools;
use rayon::{iter::IntoParallelIterator, iter::ParallelIterator};

#[derive(Debug)]
struct Mapping {
    destination_start: u64,
    source_start: u64,
    len: u64,
}

impl Mapping {
    fn convert(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source >= self.source_start + self.len {
            return None;
        }
        let offset = source - self.source_start;
        Some(self.destination_start + offset)
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn convert(&self, source: u64) -> u64 {
        self.mappings
            .iter()
            .filter_map(|m| m.convert(source))
            .next()
            .unwrap_or(source)
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day5/input.txt").expect("Unable to read file");

    let mut lines = input.lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    // skip empty line
    lines.next().unwrap();

    // Read maps
    let mut maps = Vec::new();
    while let Some(_) = lines.next() {
        let mut ranges = Vec::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            ranges.push(line);
        }
        maps.push(Map {
            mappings: ranges
                .into_iter()
                .map(|r| {
                    let (destination_start, source_start, len) =
                        r.split_whitespace().collect_tuple().unwrap();
                    Mapping {
                        destination_start: destination_start.parse().unwrap(),
                        source_start: source_start.parse().unwrap(),
                        len: len.parse().unwrap(),
                    }
                })
                .collect(),
        });
    }

    // Run the seeds through the maps
    let lowest_location = seeds
        .iter()
        .copied()
        .map(|seed| maps.iter().fold(seed, |seed, map| map.convert(seed)))
        .min();

    println!("Solution 1: {}", lowest_location.unwrap());

    // Run the seeds through the maps
    let lowest_location = seeds
        .iter()
        .copied()
        .tuples()
        .collect::<Vec<(_, _)>>()
        .into_par_iter()
        .flat_map(|(a, b)| (a..a + b).into_par_iter())
        .map(|seed| maps.iter().fold(seed, |seed, map| map.convert(seed)))
        .min();

    println!("Solution 2: {}", lowest_location.unwrap());
}
