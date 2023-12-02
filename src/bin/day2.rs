use std::ops::Add;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

impl Add<&Set> for Set {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            blue: self.blue + other.blue,
            red: self.red + other.red,
            green: self.green + other.green,
        }
    }
}

impl Set {
    fn max(&self, other: &Self) -> Self {
        Self {
            blue: self.blue.max(other.blue),
            red: self.red.max(other.red),
            green: self.green.max(other.green),
        }
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game, sets) = line.split_once(':').unwrap();
        let id = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let sets = sets
            .split(';')
            .map(|set| {
                let mut result = Set::default();
                for cubes in set.trim().split(',') {
                    let (count, color) = cubes.trim().split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    match color {
                        "blue" => result.blue += count,
                        "red" => result.red += count,
                        "green" => result.green += count,
                        _ => panic!("Unknown color {color}"),
                    }
                }
                result
            })
            .collect::<Vec<Set>>();
        Self { id, sets }
    }

    fn cubes_required(&self) -> Set {
        self.sets
            .iter()
            .fold(Set::default(), |acc, set| acc.max(set))
    }

    fn is_possible(&self, max_cubes: &Set) -> bool {
        self.sets.iter().all(|set| {
            set.blue <= max_cubes.blue && set.red <= max_cubes.red && set.green <= max_cubes.green
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day2/input.txt").expect("Unable to read file");

    let games = input.lines().map(Game::parse).collect::<Vec<Game>>();

    let possible_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let impossible_ids: u32 = games
        .iter()
        .filter(|game| game.is_possible(&possible_set))
        .map(|game| game.id)
        .sum();
    println!("Solution 1: {}", impossible_ids);

    let total_power: u32 = games.iter().map(|g| g.cubes_required().power()).sum();
    println!("Solution 2: {}", total_power);
}
