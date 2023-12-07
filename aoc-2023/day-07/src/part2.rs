use std::cmp::Ordering;

use indicatif::ProgressIterator;

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: u64,
    strength: u64,
}
enum Card {
    HighCards,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn calculate_strength(&self, cards_type: &[char; 13]) -> u64 {
        let mut result = Card::HighCards;
        let card_count = cards_type
            .iter()
            .map(|card| self.cards.iter().filter(|&c| c == card).count())
            .collect::<Vec<usize>>();
        if card_count.iter().any(|&n| n == 5) {
            result = Card::FiveOfAKind;
        } else if card_count.iter().any(|&n| n == 4) {
            match self.cards.iter().filter(|&c| *c == 'J').count() {
                1 | 4 => result = Card::FiveOfAKind,
                _ => result = Card::FourOfAKind,
            }
        } else if card_count.iter().any(|&n| n == 3) {
            if card_count.iter().any(|&n| n == 2) {
                match self.cards.iter().filter(|&c| *c == 'J').count() {
                    2 | 3 => result = Card::FiveOfAKind,
                    _ => result = Card::FullHouse,
                }
            } else {
                match self.cards.iter().filter(|&c| *c == 'J').count() {
                    1 | 3 => result = Card::FourOfAKind,
                    _ => result = Card::ThreeOfAKind,
                }
            }
        } else if card_count.iter().filter(|&n| *n == 2).count() == 2 {
            match self.cards.iter().filter(|&c| *c == 'J').count() {
                1 => result = Card::FullHouse,
                2 => result = Card::FourOfAKind,
                _ => result = Card::TwoPair,
            }
        } else if card_count.iter().any(|&n| n == 2) {
            match self.cards.iter().filter(|&c| *c == 'J').count() {
                1 | 2 => result = Card::ThreeOfAKind,
                _ => result = Card::OnePair,
            }
        } else if self.cards.iter().any(|&c| c == 'J') {
            result = Card::OnePair;
        }
        match result {
            Card::FiveOfAKind => 6,
            Card::FourOfAKind => 5,
            Card::FullHouse => 4,
            Card::ThreeOfAKind => 3,
            Card::TwoPair => 2,
            Card::OnePair => 1,
            Card::HighCards => 0,
        }
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
                .parse::<u64>()
                .expect("parse bid ok");
            Hand {
                cards,
                bid,
                strength: 0u64,
            }
        })
        .collect::<Vec<Hand>>()
}

pub fn process(input: &str) -> String {
    let cards_type = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    let mut hands = parse(input);
    hands
        .iter_mut()
        .progress()
        .for_each(|hand| hand.strength = hand.calculate_strength(&cards_type));
    hands.sort_by(|a, b| a.order(b, &cards_type));
    // dbg!(&hands);
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.bid)
        .sum::<u64>()
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
        assert_eq!("5905", process(input))
    }
    #[test]
    fn tests_case2() {
        let input = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43
";
        assert_eq!("1369", process(input))
    }
}
