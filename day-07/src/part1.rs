use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    character::complete::{anychar, line_ending, space1, u64},
    combinator::map,
    multi::{many_till, separated_list1},
    sequence::tuple,
    Finish, IResult,
};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut hands = parse_game(input)?;
    hands.sort();
    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum();
    Ok(total.to_string())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: HandCards,
    pub bid: u64,
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let card_counts = self.cards.0.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|count| *count += 1u8).or_insert(1);
            acc
        });

        match card_counts.values().collect_vec() {
            v if v.contains(&&5) => HandType::FiveOfKind,
            v if v.contains(&&4) => HandType::FourOfKind,
            v if v.contains(&&3) && v.contains(&&2) => HandType::FullHouse,
            v if v.contains(&&3) => HandType::ThreeOfKind,
            v if v.iter().filter(|v| v == &&&2u8).count() == 2 => HandType::TwoPair,
            v if v.contains(&&2) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                for (c, o) in self.cards.0.iter().zip(&other.cards.0) {
                    match c.cmp(o) {
                        std::cmp::Ordering::Equal => continue,
                        v => return v,
                    }
                }
                std::cmp::Ordering::Equal
            }
            v => v,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HandCards(pub Vec<Card>);

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Joker),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            v => Err(anyhow!("Invalid card: {:?}", v)),
        }
    }
}

pub fn parse_game(input: &str) -> anyhow::Result<Vec<Hand>> {
    let (_, game) = separated_list1(line_ending, hand)(input)
        .finish()
        .map_err(|e| anyhow!("Could not parse the game: {}", e))?;

    Ok(game)
}

pub fn hand(input: &str) -> IResult<&str, Hand> {
    map(
        tuple((many_till(card, space1), u64)),
        |((cards, _), bid)| Hand {
            cards: HandCards(cards),
            bid,
        },
    )(input)
}

pub fn card(input: &str) -> IResult<&str, Card> {
    map(anychar, |ch| ch.try_into().unwrap())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_card() -> anyhow::Result<()> {
        assert_eq!(("", Card::Ace), card("A")?);
        assert_eq!(("", Card::King), card("K")?);
        assert_eq!(("", Card::Queen), card("Q")?);
        assert_eq!(("", Card::Joker), card("J")?);
        assert_eq!(("", Card::Ten), card("T")?);
        assert_eq!(("", Card::Nine), card("9")?);
        assert_eq!(("", Card::Eight), card("8")?);
        assert_eq!(("", Card::Seven), card("7")?);
        assert_eq!(("", Card::Six), card("6")?);
        assert_eq!(("", Card::Five), card("5")?);
        assert_eq!(("", Card::Four), card("4")?);
        assert_eq!(("", Card::Three), card("3")?);
        assert_eq!(("", Card::Two), card("2")?);
        Ok(())
    }

    #[test]
    fn test_parse_hand() -> anyhow::Result<()> {
        let (rest, hand) = hand("T55J5 684")?;
        assert_eq!("", rest);
        assert_eq!(
            vec![Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five],
            hand.cards.0
        );
        assert_eq!(684, hand.bid);
        Ok(())
    }

    #[test]
    fn test_hand_type() -> anyhow::Result<()> {
        assert_eq!(
            HandType::ThreeOfKind,
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Ten,
                    Card::Five,
                    Card::Five,
                    Card::Joker,
                    Card::Five
                ])
            }
            .hand_type()
        );

        assert_eq!(
            HandType::FiveOfKind,
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Five,
                    Card::Five,
                    Card::Five,
                    Card::Five,
                    Card::Five
                ])
            }
            .hand_type()
        );

        assert_eq!(
            HandType::FullHouse,
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Five,
                    Card::Two,
                    Card::Two,
                    Card::Five,
                    Card::Five
                ])
            }
            .hand_type()
        );

        assert_eq!(
            HandType::TwoPair,
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Joker,
                    Card::Five,
                    Card::Two,
                    Card::Five,
                    Card::Two
                ])
            }
            .hand_type()
        );

        assert_eq!(
            HandType::OnePair,
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Joker,
                    Card::Five,
                    Card::King,
                    Card::Five,
                    Card::Two
                ])
            }
            .hand_type()
        );

        Ok(())
    }

    #[test]
    fn test_compare_hands() {
        assert!(
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Joker,
                    Card::Five,
                    Card::King,
                    Card::Five,
                    Card::Two
                ])
            } < Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Joker,
                    Card::Five,
                    Card::Two,
                    Card::Five,
                    Card::Two
                ])
            }
        );

        assert!(
            Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::King,
                    Card::Five,
                    Card::King,
                    Card::Five,
                    Card::Two
                ])
            } > Hand {
                bid: 0,
                cards: HandCards(vec![
                    Card::Joker,
                    Card::Five,
                    Card::Two,
                    Card::Five,
                    Card::Two
                ])
            }
        );
    }
}
