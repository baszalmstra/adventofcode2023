use itertools::Itertools;

struct Card {
    winning: Vec<u32>,
    deck: Vec<u32>,
}

fn parse_card(line: &str) -> Card {
    let (_, rest) = line.split_once(':').unwrap();
    let (winning, rest) = rest.split_once('|').unwrap();
    let winning = winning
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let deck = rest
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    Card { winning, deck }
}

impl Card {
    pub fn matches(&self) -> u32 {
        self.deck
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }

    pub fn score(&self) -> u32 {
        let num_matching = self.matches();
        if num_matching > 0 {
            1 << (num_matching - 1)
        } else {
            0
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day4/input.txt").expect("Unable to read file");

    let cards = input.lines().map(parse_card).collect_vec();
    let solution1 = cards.iter().map(Card::score).sum::<u32>();
    println!("Solution 1: {solution1}");

    let mut counts = vec![1usize; cards.len()];
    for i in 0..cards.len() {
        let card = &cards[i];
        let score = card.matches();
        let count = counts[i];
        for j in 0..score as usize {
            let idx = i + j + 1;
            if idx < cards.len() {
                counts[idx] += count;
            }
        }
    }
    let solution2 = counts.iter().sum::<usize>();
    println!("Solution 2: {solution2}");
}
