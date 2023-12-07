use std::{cmp::Ordering, collections::HashMap};

///
/// Cards from strongest to weakest.
///
const CARDS: &[char] = &[
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn solution(input: &str) -> u32 {
    let mut camel_cards = CamelCards::from_input(input);
    camel_cards.games.sort();

    camel_cards
        .games
        .iter()
        .enumerate()
        .fold(0, |acc, (index, game)| {
            acc + (game.bid * (index as u32 + 1))
        })
}

///
/// Camel Cards.
///
#[derive(Debug)]
struct CamelCards {
    games: Vec<Game>,
}

impl CamelCards {
    fn from_input(input: &str) -> Self {
        let games: Vec<Game> = input
            .lines()
            .map(|line| {
                let (cards, bid) = line
                    .split_once(" ")
                    .expect("Line should always be splitable");

                let cards = cards.chars().map(|char| Card(char)).collect();
                let bid = bid.parse().expect("Bid should always be parsable");
                Game { cards, bid }
            })
            .collect();

        Self { games }
    }
}

///
/// Single card game.
///
#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
    bid: u32,
}

impl Game {
    fn get_type(&self) -> GameType {
        let mut counts: HashMap<char, u8> = HashMap::new();

        for card in &self.cards {
            *counts.entry(card.0).or_insert(0) += 1;
        }

        let jokers_count = counts.get(&'J').unwrap_or(&0);

        if jokers_count == &5 {
            return GameType::FiveOfAKind;
        }

        // Two pair check
        if !self.cards.contains(&Card('J')) {
            if counts
                .values()
                .filter(|count| (*count + jokers_count) == 2_u8)
                .count()
                == 2
            {
                return GameType::TwoPair;
            }
        }

        // Full house check
        if counts.iter().filter(|(card, _)| *card != &'J').count() == 2 {
            let mut full_house = true;

            for (card, count) in &counts {
                if card == &'J' {
                    continue;
                }

                if (count + jokers_count) != 2 && (count + jokers_count) != 3 {
                    full_house = false;
                    break;
                }
            }

            if full_house {
                return GameType::FullHouse;
            }
        }

        counts.iter().filter(|(card, _)| *card != &'J').fold(
            GameType::HighCard,
            |prev_type, (_, cur_count)| {
                let cur_type = match cur_count + jokers_count {
                    1 => GameType::HighCard,
                    2 => GameType::OnePair,
                    3 => GameType::ThreeOfAKind,
                    4 => GameType::FourOfAKind,
                    5 => GameType::FiveOfAKind,
                    _ => unreachable!("Impossible to get here"),
                };

                if cur_type > prev_type {
                    return cur_type;
                }

                return prev_type;
            },
        )
    }
}

impl Eq for Game {}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false;
        }

        for index in 0..self.cards.len() {
            if self.cards[index].0 != other.cards[index].0 {
                return false;
            }
        }

        return true;
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other)
            .expect("Game should always compare to another game")
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_type = self.get_type();
        let other_type = other.get_type();

        if self_type == other_type {
            for index in 0..self.cards.len() {
                if self.cards[index] == other.cards[index] {
                    continue;
                }

                return Some(self.cards[index].cmp(&other.cards[index]));
            }
        }

        return Some(self_type.cmp(&other_type));
    }
}

///
/// Game type.
///
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum GameType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

///
/// Single game card.
///
#[derive(Eq, PartialEq)]
struct Card(char);

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other)
            .expect("Card should always compare to another card")
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_strength = CARDS
            .iter()
            .position(|card| card == &self.0)
            .expect("Card strength should always be obtained");
        let other_strength = CARDS
            .iter()
            .position(|card| card == &other.0)
            .expect("Card strength should always be obtained");

        Some(other_strength.cmp(&self_strength))
    }
}
