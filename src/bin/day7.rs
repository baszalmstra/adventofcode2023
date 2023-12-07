use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    kind: Option<HandKind>,
    cards: [Card; 5],
    score: u32,
}

impl Hand {
    fn cmp(&self, other: &Self, with_joker: bool) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match (a, b) {
                        (Card::Jack, Card::Jack) => continue,
                        (Card::Jack, _) if with_joker => return Ordering::Less,
                        (_, Card::Jack) if with_joker => return Ordering::Equal,
                        (a, b) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            o => return o,
                        },
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

fn parse_hand(hand: &str, with_joker: bool) -> Hand {
    let (cards, score) = hand.split_once(' ').unwrap();

    let cards: [Card; 5] = cards
        .chars()
        .map(|c| Card::try_from(c).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let x = move |a, b| a == b || (with_joker && b == &Card::Jack);

    let kind = cards
        .iter()
        .permutations(5)
        .map(|p| p.into_iter().collect_tuple().unwrap())
        .filter_map(|(a, b, c, d, e)| {
            if x(a, b) && x(a, c) && x(a, d) && x(a, e) {
                Some(HandKind::FiveOfAKind)
            } else if x(a, b) && x(a, c) && x(a, d) {
                Some(HandKind::FourOfAKind)
            } else if x(a, b) && x(a, c) && x(d, e) {
                Some(HandKind::FullHouse)
            } else if x(a, b) && x(a, c) {
                Some(HandKind::ThreeOfAKind)
            } else if x(a, b) && x(c, d) {
                Some(HandKind::TwoPair)
            } else if x(a, b) {
                Some(HandKind::OnePair)
            } else if a != b && b != c && c != d && d != e {
                Some(HandKind::HighCard)
            } else {
                None
            }
        })
        .max();

    Hand {
        cards,
        kind,
        score: score.parse().unwrap(),
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day7/input.txt").unwrap();
    let line = input
        .lines()
        .map(|l| parse_hand(l, false))
        .sorted_by(|a, b| a.cmp(b, false))
        .collect_vec();

    let solution1: u32 = line
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx as u32 + 1) * h.score)
        .sum();
    println!("Solution 1: {solution1}");

    let line = input
        .lines()
        .map(|l| parse_hand(l, true))
        .sorted_by(|a, b| a.cmp(b, true))
        .collect_vec();

    let solution2: u32 = line
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx as u32 + 1) * h.score)
        .sum();
    println!("Solution 2: {solution2}");
}
