fn main() {
    let input = std::fs::read_to_string("inputs/day1/input.txt").expect("Unable to read file");

    let result: usize = input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            (first * 10 + last) as usize
        })
        .sum();
    println!("Result: {result}");

    let result: usize = input
        .lines()
        .map(|line| {
            let first = [
                "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
                "seven", "7", "eight", "8", "nine", "9",
            ]
            .iter()
            .enumerate()
            .fold(None, |acc, (idx, word)| {
                let digit = (idx / 2) + 1;
                let pos = line.find(word);
                match (acc, pos) {
                    (None, Some(pos)) => Some((pos, digit)),
                    (Some((pos, _)), Some(this_pos)) if this_pos < pos => Some((this_pos, digit)),
                    (acc, _) => acc,
                }
            })
            .unwrap()
            .1;
            let last = [
                "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
                "seven", "7", "eight", "8", "nine", "9",
            ]
            .iter()
            .enumerate()
            .fold(None, |acc, (idx, word)| {
                let digit = (idx / 2) + 1;
                let pos = line.rfind(word);
                match (acc, pos) {
                    (None, Some(pos)) => Some((pos, digit)),
                    (Some((pos, _)), Some(this_pos)) if this_pos > pos => Some((this_pos, digit)),
                    (acc, _) => acc,
                }
            })
            .unwrap()
            .1;
            first * 10 + last
        })
        .sum();
    println!("Result2: {result}");
}
