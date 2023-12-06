use itertools::Itertools;

struct Race {
    time: u64,
    max_distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        let mut ways = 0;
        for i in 0..=self.time {
            let distance = (self.time - i) * i;
            if distance > self.max_distance {
                ways += 1;
            }
        }
        ways
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day6/input.txt").expect("Unable to read file");

    let (time, distance) = input.lines().collect_tuple().unwrap();
    let times: Vec<_> = time
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let distances: Vec<_> = distance
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let races = times
        .into_iter()
        .zip_eq(distances)
        .map(|(time, max_distance)| Race { time, max_distance })
        .collect_vec();

    let solution1: u64 = races.iter().map(|race| race.ways_to_win()).product();
    println!("Solution 1: {solution1}");

    let single_race = Race {
        time: time
            .strip_prefix("Time:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap(),
        max_distance: distance
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap(),
    };
    println!("Solution 2: {}", single_race.ways_to_win());
}
