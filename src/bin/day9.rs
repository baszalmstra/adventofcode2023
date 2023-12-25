use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("inputs/day9/input.txt").unwrap();

    let lines = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(i64::from_str)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect_vec();

    let solution1: i64 = lines.iter().map(|history| predict(history, false)).sum();
    println!("Solution 1: {solution1}");

    let solution2: i64 = lines.iter().map(|history| predict(history, true)).sum();
    println!("Solution 2: {solution2}");
}

fn predict(history: &[i64], front: bool) -> i64 {
    let mut derived = Vec::with_capacity(history.len() - 1);
    let mut non_zero = false;
    for (a, b) in history.iter().tuple_windows() {
        if b - a != 0 {
            non_zero = true;
        }
        derived.push(b - a);
    }

    match (non_zero, front) {
        (true, true) => history[0] - predict(&derived, front),
        (true, false) => history[history.len() - 1] + predict(&derived, front),
        (false, true) => history[0],
        (false, false) => history[history.len() - 1],
    }
}
