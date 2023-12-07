use std::cmp::Ordering;

use indicatif::ProgressIterator;

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: u32,
    strength: u32,
}

impl Hand {
    fn calculate_strength(&self, cards_type: &[char; 13]) -> u32 {
        let card_count = cards_type
            .iter()
            .map(|card| self.cards.iter().filter(|&c| c == card).count())
            .collect::<Vec<usize>>();
        if card_count.iter().any(|&n| n == 5) {
            return 6;
        }
        if card_count.iter().any(|&n| n == 4) {
            return 5;
        }
        if card_count.iter().any(|&n| n == 3) {
            if card_count.iter().any(|&n| n == 2) {
                return 4;
            } else {
                return 3;
            }
        }
        if card_count.iter().filter(|&n| *n == 2).count() == 2 {
            return 2;
        } else if card_count.iter().any(|&n| n == 2) {
            return 1;
        }
        0
    }
    fn order(&self, other: &Hand, cards_type: &[char; 13]) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let s = self
                    .cards
                    .iter()
                    .map(|card| {
                        cards_type
                            .iter()
                            .position(|c| c == card)
                            .expect("value of card")
                    })
                    .collect::<Vec<usize>>();
                let o = other
                    .cards
                    .iter()
                    .map(|card| {
                        cards_type
                            .iter()
                            .position(|c| c == card)
                            .expect("value of card")
                    })
                    .collect::<Vec<usize>>();
                if let Some(id) = s.iter().zip(o.iter()).position(|(s, o)| s > o || o > s) {
                    if s[id] < o[id] {
                        //Smaller is stronger
                        return Ordering::Greater;
                    } else {
                        return Ordering::Less;
                    }
                }
                dbg!("Some thing wrong here");
                Ordering::Equal
            }
        }
    }
}

fn parse(input: &str) -> Vec<Hand> {
    let hands = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    hands
        .iter()
        .map(|hand| {
            let cards: [char; 5] = hand
                .first()
                .expect("get cards")
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("parse card");
            let bid = hand
                .last()
                .expect("get bid")
                .parse::<u32>()
                .expect("parse bid ok");
            Hand {
                cards,
                bid,
                strength: 0u32,
            }
        })
        .collect::<Vec<Hand>>()
}

pub fn process(input: &str) -> String {
    let cards_type = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let mut hands = parse(input);
    hands
        .iter_mut()
        .progress()
        .for_each(|hand| hand.strength = hand.calculate_strength(&cards_type));
    hands.sort_unstable_by(|a, b| a.order(b, &cards_type));
    // dbg!(&hands);
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!("6440", process(input))
    }
}
