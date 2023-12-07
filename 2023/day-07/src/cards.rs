use std::{collections::HashMap, str::FromStr};

use strum_macros::EnumString;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumString, Clone, Copy, Hash)]
pub enum Card {
    #[strum(serialize = "J")]
    Joker,

    #[strum(serialize = "2")]
    Two,

    #[strum(serialize = "3")]
    Three,

    #[strum(serialize = "4")]
    Four,

    #[strum(serialize = "5")]
    Five,

    #[strum(serialize = "6")]
    Six,

    #[strum(serialize = "7")]
    Seven,

    #[strum(serialize = "8")]
    Eight,

    #[strum(serialize = "9")]
    Nine,

    #[strum(serialize = "T")]
    Ten,

    // #[strum(serialize = "J")]
    // Jack,
    #[strum(serialize = "Q")]
    Queen,

    #[strum(serialize = "K")]
    King,

    #[strum(serialize = "A")]
    Ace,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[Card; 5]> for HandRank {
    fn from(cards: [Card; 5]) -> Self {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in cards {
            if card == Card::Joker {
                continue;
            }
            match map.get_mut(&card) {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    map.insert(card, 1);
                }
            }
        }

        let mut counts: Vec<_> = map.values().collect();
        counts.sort();

        return match counts[..] {
            [5] => HandRank::FiveOfAKind,
            [1, 4] => HandRank::FourOfAKind,
            [2, 3] => HandRank::FullHouse,
            [1, 1, 3] => HandRank::ThreeOfAKind,
            [1, 2, 2] => HandRank::TwoPair,
            [1, 1, 1, 2] => HandRank::OnePair,
            [1, 1, 1, 1, 1] => HandRank::HighCard,

            // Matches w/ jokers
            [] | [1] | [2] | [3] | [4] => HandRank::FiveOfAKind,
            [1, 1] | [1, 2] | [1, 3] => HandRank::FourOfAKind,
            [1, 1, 1] | [1, 1, 2] => HandRank::ThreeOfAKind,
            [2, 2] => HandRank::FullHouse,
            [1, 1, 1, 1] => HandRank::OnePair,

            _ => panic!("couldn't find hand type for {counts:?}"),
        };
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Hand {
    // Fields must stay in this order to use "natrual" sort order
    rank: HandRank,
    cards: [Card; 5],
    pub bid: u32,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: u32) -> Hand {
        Hand {
            cards: cards,
            rank: HandRank::from(cards),
            bid: bid,
        }
    }
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();

        let card_str = parts[0];
        let bid = match parts[1].parse::<u32>() {
            Ok(bid) => bid,
            Err(e) => panic!("bid is not a number: {e}"),
        };

        if card_str.len() != 5 {
            panic!("hand must be 5 characters long");
        }

        let mut cards: [Option<Card>; 5] = [None, None, None, None, None];
        for (pos, char) in card_str.chars().enumerate() {
            match Card::from_str(char.to_string().as_str()) {
                Ok(card) => cards[pos] = Some(card),
                Err(error) => {
                    panic!("invalid card {char}: {error}");
                }
            }
        }

        Hand::new(
            [
                cards[0].take().unwrap(),
                cards[1].take().unwrap(),
                cards[2].take().unwrap(),
                cards[3].take().unwrap(),
                cards[4].take().unwrap(),
            ],
            bid,
        )
    }
}

impl From<&String> for Hand {
    fn from(line: &String) -> Self {
        Hand::from(line.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::{Card, Hand, HandRank};

    #[test]
    fn test_hand() {
        let rows = ["AATQ3 2689"].map(String::from).to_vec();

        let hand = Hand::from(&rows[0]);

        println!("{:?}", hand);
        assert_eq!(hand.bid, 2689);
        assert_eq!(hand.rank, HandRank::OnePair);
        assert_eq!(
            hand.cards,
            [Card::Ace, Card::Ace, Card::Ten, Card::Queen, Card::Three]
        );
    }

    #[test]
    fn test_card_eq() {
        assert_eq!(Card::Ace > Card::Nine, true);
        assert_eq!(Card::Three > Card::Queen, false);

        assert_eq!(Card::Joker < Card::Two, true);
    }
}
