use std::collections::HashMap;

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let bag = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let sum: u32 = input
        .lines()
        .filter_map(|line| parse_game(line).ok())
        .filter(|game| game.is_valid(&bag))
        .map(|game| game.id)
        .sum();

    Ok(sum.to_string())
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<HashMap<String, u32>>,
}

impl Game {
    pub fn is_valid(&self, bag: &HashMap<String, u32>) -> bool {
        self.subsets.iter().all(|v| {
            v.iter().all(|(colour, count)| match bag.get(colour) {
                None => false,
                Some(max_available) => max_available >= count,
            })
        })
    }
}

pub fn parse_game(input: &str) -> anyhow::Result<Game> {
    let (rest, id) = preceded(
        tag("Game "),
        map(
            terminated(digit1::<&str, nom::error::Error<&str>>, tag(": ")),
            |v: &str| v.parse::<u32>().unwrap(),
        ),
    )(input)
    .map_err(|e| anyhow!("Failed to parse game ID: {}", e.to_string()))?;
    let (_, subsets) = separated_list1(tag("; "), parse_subset)(rest)
        .finish()
        .map_err(|e| anyhow!("Failed to parse game subsets: {}", e.to_string()))?;

    Ok(Game { id, subsets })
}

fn parse_subset(input: &str) -> IResult<&str, HashMap<String, u32>> {
    map(
        separated_list1(
            tag(", "),
            map(
                separated_pair(digit1::<&str, nom::error::Error<&str>>, tag(" "), alpha1),
                |(count, color)| (color.to_string(), count.parse::<u32>().unwrap()),
            ),
        ),
        |v| v.into_iter().collect(),
    )(input)
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_subset() {
        let res = parse_subset("3 blue, 4 red").unwrap();
        assert_eq!(&3, res.1.get("blue").unwrap());
        assert_eq!(&4, res.1.get("red").unwrap());
    }

    #[test]
    fn test_parse_game() {
        let game =
            parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
        assert_eq!(2, game.id);
        assert_equal(
            vec![
                HashMap::from([("blue".to_string(), 1), ("green".to_string(), 2)]),
                HashMap::from([
                    ("blue".to_string(), 4),
                    ("green".to_string(), 3),
                    ("red".to_string(), 1),
                ]),
                HashMap::from([("blue".to_string(), 1), ("green".to_string(), 1)]),
            ],
            game.subsets,
        );
    }

    #[test]
    fn test_game_invalid() {
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        let bag = &HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]);
        let game = Game {
            id: 3,
            subsets: vec![
                HashMap::from([
                    ("green".to_string(), 8),
                    ("blue".to_string(), 6),
                    ("red".to_string(), 20),
                ]),
                HashMap::from([
                    ("blue".to_string(), 5),
                    ("red".to_string(), 4),
                    ("green".to_string(), 13),
                ]),
                HashMap::from([("green".to_string(), 5), ("red".to_string(), 1)]),
            ],
        };
        assert!(!game.is_valid(bag));
    }

    #[test]
    fn test_game_valid() {
        let bag = &HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]);
        let game = Game {
            id: 1,
            subsets: vec![
                HashMap::from([("blue".to_string(), 3), ("red".to_string(), 4)]),
                HashMap::from([
                    ("blue".to_string(), 6),
                    ("red".to_string(), 1),
                    ("green".to_string(), 2),
                ]),
                HashMap::from([("green".to_string(), 2)]),
            ],
        };
        assert!(game.is_valid(bag));
    }
}
