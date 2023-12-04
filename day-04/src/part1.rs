use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
};
use std::collections::HashSet;

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let total_points: u32 = input
        .lines()
        .flat_map(parse_card)
        .map(|card| card.points())
        .sum();
    Ok(total_points.to_string())
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: HashSet<u32>,
    pub card_numbers: HashSet<u32>,
}

impl Card {
    pub fn matches(&self) -> HashSet<u32> {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .cloned()
            .collect()
    }

    pub fn points(&self) -> u32 {
        let matching = self.matches();

        match matching.len() {
            0 => 0,
            1 => 1,
            len => 2u32.pow(len as u32 - 1),
        }
    }
}

pub fn parse_card(input: &str) -> anyhow::Result<Card> {
    let (rest, id) = delimited(
        tuple((tag("Card"), space1)),
        u32::<_, nom::error::Error<_>>,
        tuple((tag(":"), space1)),
    )(input)
    .map_err(|e: nom::Err<_>| anyhow!("Could not parse card ID: {}", e))?;

    let (_, (winning_numbers, card_numbers)) = separated_pair(
        separated_list1(space1, u32::<_, nom::error::Error<_>>),
        tuple((space1, tag("|"), space1)),
        separated_list1(space1, u32),
    )(rest)
    .map_err(|e: nom::Err<_>| anyhow!("Could not parse card numbers: {}", e))?;

    Ok(Card {
        id,
        winning_numbers: HashSet::from_iter(winning_numbers),
        card_numbers: HashSet::from_iter(card_numbers),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_card() {
        let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        assert_eq!(1, card.id);
        assert_eq!(
            HashSet::from_iter(vec![41, 48, 83, 86, 17]),
            card.winning_numbers
        );
        assert_eq!(
            HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
            card.card_numbers
        );
    }

    #[test]
    fn test_matching_numbers() {
        let card = Card {
            id: 1,
            winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
            card_numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
        };
        assert_eq!(HashSet::from_iter(vec![48, 83, 17, 86]), card.matches());
    }

    #[test]
    fn test_points_calculation() {
        let card = Card {
            id: 1,
            winning_numbers: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
            card_numbers: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
        };
        assert_eq!(8, card.points());
    }
}
