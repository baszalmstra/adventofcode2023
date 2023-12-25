use itertools::Itertools;
use std::collections::HashMap;

enum LeftRight {
    Left,
    Right,
}

fn main() {
    let input = std::fs::read_to_string("inputs/day8/input.txt").unwrap();

    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next().unwrap();

    let instrs = instructions
        .chars()
        .map(|c| match c {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let graph: HashMap<&str, (&str, &str)> = HashMap::from_iter(lines.map(|line| {
        let (node, left_right) = line.split_once(" = ").unwrap();
        let (left, right) = left_right
            .trim_matches(&['(', ')'][..])
            .split_once(", ")
            .unwrap();
        (node, (left, right))
    }));

    let steps = count_steps(&graph, &instrs, "AAA");
    println!("Solution 1: {steps}");

    let steps = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|key| count_steps(&graph, &instrs, key))
        .reduce(|a, b| num::integer::lcm(b, a))
        .unwrap();
    println!("Solution 2: {steps}");
}

fn count_steps<'a>(
    graph: &HashMap<&'a str, (&'a str, &'a str)>,
    instructions: &[LeftRight],
    mut current: &'a str,
) -> u64 {
    let mut instructions = instructions.iter().cycle();
    let mut steps = 0;
    loop {
        steps += 1;
        let (left, right) = graph.get(current).unwrap();
        current = match instructions.next() {
            None => unreachable!(),
            Some(LeftRight::Left) => left,
            Some(LeftRight::Right) => right,
        };
        if current.ends_with('Z') {
            return steps;
        }
    }
}
