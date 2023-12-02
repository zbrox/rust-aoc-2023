use std::collections::HashMap;

use itertools::Itertools;

use crate::part1::parse_game;

#[tracing::instrument]
pub fn solve(input: &str) -> anyhow::Result<String> {
    let sum: u32 = input
        .lines()
        .filter_map(|line| parse_game(line).ok())
        .map(|game| calculate_power(&game.subsets))
        .sum();
    Ok(sum.to_string())
}

fn calculate_power(subsets: &[HashMap<String, u32>]) -> u32 {
    let max_by_colour: HashMap<String, u32> = subsets
        .iter()
        .flat_map(|v| v.iter())
        .sorted()
        .group_by(|v| v.0)
        .into_iter()
        .map(|(colour, group)| {
            (
                colour.to_string(),
                group.map(|(_, count)| *count).max().unwrap(),
            )
        })
        .collect();

    max_by_colour.values().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", solve(input)?);
        Ok(())
    }
}
